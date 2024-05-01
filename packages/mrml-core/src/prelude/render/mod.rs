use std::convert::TryFrom;

use crate::helper::size::{Pixel, Size};
use crate::helper::spacing::Spacing;
use std::sync::atomic::{AtomicU16, Ordering};

mod buffer;
mod header;
mod options;
mod tag;

pub use buffer::*;
pub use header::*;
pub use options::*;
pub use tag::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown fragment {0}")]
    UnknownFragment(String),
}

#[derive(Debug, Default)]
pub struct Generator(AtomicU16);

impl Generator {
    pub fn next_id(&self) -> String {
        let id = self.0.fetch_add(1, Ordering::SeqCst);
        format!("{id:0>8}")
    }
}

#[deprecated = "use mrml::prelude::render::RenderOptions instead"]
pub type Options = RenderOptions;

pub struct RenderContext<'h> {
    pub options: &'h RenderOptions,
    pub header: Header<'h>,
    pub generator: Generator,
}

impl<'h> RenderContext<'h> {
    pub fn new(options: &'h RenderOptions, header: Header<'h>) -> Self {
        Self {
            options,
            header,
            generator: Generator::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct RenderCursor {
    pub buffer: RenderBuffer,
    pub header: VariableHeader,
}

pub(crate) struct Renderer<'element, 'header, Element, Extra> {
    pub context: &'header RenderContext<'header>,
    pub element: &'element Element,
    pub extra: Extra,
}

impl<'element, 'header, Element, Extra> Renderer<'element, 'header, Element, Extra> {
    #[inline]
    pub fn new(
        context: &'header RenderContext<'header>,
        element: &'element Element,
        extra: Extra,
    ) -> Self {
        Self {
            context,
            element,
            extra,
        }
    }
}

pub trait Render<'element, 'header> {
    fn context(&self) -> &'header RenderContext<'header>;

    fn tag(&self) -> Option<&str> {
        None
    }

    fn raw_attribute(&self, _: &str) -> Option<&'element str> {
        None
    }

    fn raw_extra_attribute(&self, _: &str) -> Option<&str> {
        None
    }

    fn attribute_as_pixel(&self, name: &str) -> Option<Pixel> {
        self.attribute(name)
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn attribute_as_size(&self, name: &str) -> Option<Size> {
        self.attribute(name)
            .and_then(|value| Size::try_from(value.as_str()).ok())
    }

    fn attribute_as_spacing(&self, name: &str) -> Option<Spacing> {
        self.attribute(name)
            .and_then(|value| Spacing::try_from(value.as_str()).ok())
    }

    fn attribute_equals(&self, key: &str, value: &str) -> bool {
        self.attribute(key).map(|res| res == value).unwrap_or(false)
    }

    fn attribute_exists(&self, key: &str) -> bool {
        self.attribute(key).is_some()
    }

    fn get_border_left(&self) -> Option<Pixel> {
        self.attribute_as_pixel("border-left").or_else(|| {
            self.attribute("border")
                .and_then(|value| Pixel::from_border(&value))
        })
    }

    fn get_border_right(&self) -> Option<Pixel> {
        self.attribute_as_pixel("border-right").or_else(|| {
            self.attribute("border")
                .and_then(|value| Pixel::from_border(&value))
        })
    }

    fn get_border_horizontal(&self) -> Pixel {
        let left = self.get_border_left().map(|v| v.value()).unwrap_or(0.0);
        let right = self.get_border_right().map(|v| v.value()).unwrap_or(0.0);
        Pixel::new(left + right)
    }

    fn get_inner_border_left(&self) -> Option<Pixel> {
        self.attribute_as_pixel("inner-border-left").or_else(|| {
            self.attribute_as_spacing("inner-border")
                .and_then(|s| s.left().as_pixel().cloned())
        })
    }

    fn get_inner_border_right(&self) -> Option<Pixel> {
        self.attribute_as_pixel("inner-border-right").or_else(|| {
            self.attribute_as_spacing("inner-border")
                .and_then(|s| s.right().as_pixel().cloned())
        })
    }

    fn get_padding_top(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-top").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.top().as_pixel().cloned())
        })
    }

    fn get_padding_bottom(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-bottom").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.bottom().as_pixel().cloned())
        })
    }

    fn get_padding_left(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-left").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.left().as_pixel().cloned())
        })
    }

    fn get_padding_right(&self) -> Option<Pixel> {
        self.attribute_as_pixel("padding-right").or_else(|| {
            self.attribute_as_spacing("padding")
                .and_then(|s| s.right().as_pixel().cloned())
        })
    }

    fn get_padding_horizontal(&self) -> Pixel {
        let left = self.get_padding_left().map(|v| v.value()).unwrap_or(0.0);
        let right = self.get_padding_right().map(|v| v.value()).unwrap_or(0.0);
        Pixel::new(left + right)
    }

    fn get_padding_vertical(&self) -> Pixel {
        let top = self.get_padding_top().map(|v| v.value()).unwrap_or(0.0);
        let bottom = self.get_padding_bottom().map(|v| v.value()).unwrap_or(0.0);
        Pixel::new(top + bottom)
    }

    fn get_width(&self) -> Option<Size> {
        self.attribute_as_size("width")
    }

    fn default_attribute(&self, _key: &str) -> Option<&'static str> {
        None
    }

    fn attribute(&self, key: &str) -> Option<String> {
        if let Some(value) = self.raw_attribute(key) {
            return Some(value.to_string());
        }
        if let Some(value) = self.raw_extra_attribute(key) {
            return Some(value.to_string());
        }
        let header = &self.context().header;
        if let Some(value) = self.raw_attribute("mj-class").and_then(|mj_classes| {
            mj_classes
                .split(' ')
                .map(|mj_class| mj_class.trim())
                .filter_map(|mj_class| header.attribute_class(mj_class, key))
                .next()
        }) {
            return Some(value.to_string());
        }
        if let Some(tag) = self.tag() {
            if let Some(value) = header.attribute_element(tag, key) {
                return Some(value.to_string());
            }
        }
        if let Some(value) = header.attribute_all(key) {
            return Some(value.to_string());
        }
        self.default_attribute(key).map(|item| item.to_string())
    }

    fn attribute_size(&self, key: &str) -> Option<Size> {
        self.attribute(key)
            .and_then(|value| Size::try_from(value.as_str()).ok())
    }

    fn attribute_pixel(&self, key: &str) -> Option<Pixel> {
        self.attribute(key)
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn set_style<'a>(&self, _name: &str, tag: Tag<'a>) -> Tag<'a> {
        tag
    }

    fn set_container_width(&mut self, _width: Option<Pixel>) {}
    fn set_index(&mut self, _index: usize) {}
    fn set_siblings(&mut self, _count: usize) {}
    fn set_raw_siblings(&mut self, _count: usize) {}

    fn add_extra_attribute(&mut self, _key: &str, _value: &str) {}
    fn maybe_add_extra_attribute(&mut self, key: &str, value: Option<String>) {
        if let Some(ref value) = value {
            self.add_extra_attribute(key, value);
        }
    }

    fn render_fragment(&self, name: &str, cursor: &mut RenderCursor) -> Result<(), Error> {
        match name {
            "main" => self.render(cursor),
            _ => Err(Error::UnknownFragment(name.to_string())),
        }
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error>;
}

pub trait Renderable<'render, 'element: 'render, 'header: 'render> {
    fn is_raw(&'element self) -> bool {
        false
    }

    fn renderer(
        &'element self,
        context: &'header RenderContext<'header>,
    ) -> Box<dyn Render<'element, 'header> + 'render>;
}

#[cfg(test)]
#[macro_export]
macro_rules! should_render {
    ($name: ident, $template: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, sync {
            #[cfg(feature = "parse")]
            #[test]
            fn fn_name() {
                let opts = $crate::prelude::render::RenderOptions::default();
                let template = include_str!(concat!("../../resources/compare/success/", $template, ".mjml"));
                let expected = include_str!(concat!("../../resources/compare/success/", $template, ".html"));
                let root = $crate::parse(template).unwrap();
                html_compare::assert_similar(expected, root.render(&opts).unwrap().as_str());
            }
        });
        concat_idents::concat_idents!(fn_name = $name, _, "async" {
            #[cfg(all(feature = "async", feature = "parse"))]
            #[tokio::test]
            async fn fn_name() {
                let opts = $crate::prelude::render::RenderOptions::default();
                let template = include_str!(concat!("../../resources/compare/success/", $template, ".mjml"));
                let expected = include_str!(concat!("../../resources/compare/success/", $template, ".html"));
                let root = $crate::async_parse(template).await.unwrap();
                html_compare::assert_similar(expected, root.render(&opts).unwrap().as_str());
            }
        });
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn header_should_increase() {
        let gen = super::Generator::default();
        assert_eq!(gen.next_id(), "00000000");
        assert_eq!(gen.next_id(), "00000001");
        assert_eq!(gen.next_id(), "00000002");
    }
}
