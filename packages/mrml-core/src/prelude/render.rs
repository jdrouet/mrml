use std::borrow::Cow;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;
use std::sync::atomic::{AtomicU16, Ordering};

use super::hash::Set;
use crate::helper::size::{Pixel, Size};
use crate::helper::spacing::Spacing;
use crate::helper::tag::Tag;
use crate::mj_head::MjHead;
use crate::prelude::hash::Map;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unknown fragment {0}")]
    UnknownFragment(String),
}

#[deprecated = "use mrml::prelude::render::RenderOptions instead"]
pub type Options = RenderOptions;

#[derive(Debug)]
pub struct RenderOptions {
    pub disable_comments: bool,
    pub social_icon_origin: Option<Cow<'static, str>>,
    pub fonts: HashMap<String, Cow<'static, str>>,
}

impl Default for RenderOptions {
    fn default() -> Self {
        Self {
            disable_comments: false,
            social_icon_origin: None,
            fonts: HashMap::from([
                (
                    "Open Sans".into(),
                    "https://fonts.googleapis.com/css?family=Open+Sans:300,400,500,700".into(),
                ),
                (
                    "Droid Sans".into(),
                    "https://fonts.googleapis.com/css?family=Droid+Sans:300,400,500,700".into(),
                ),
                (
                    "Lato".into(),
                    "https://fonts.googleapis.com/css?family=Lato:300,400,500,700".into(),
                ),
                (
                    "Roboto".into(),
                    "https://fonts.googleapis.com/css?family=Roboto:300,400,500,700".into(),
                ),
                (
                    "Ubuntu".into(),
                    "https://fonts.googleapis.com/css?family=Ubuntu:300,400,500,700".into(),
                ),
            ]),
        }
    }
}

pub struct Header<'h> {
    head: &'h Option<MjHead>,
    attributes_all: Map<&'h str, &'h str>,
    attributes_class: Map<&'h str, Map<&'h str, &'h str>>,
    attributes_element: Map<&'h str, Map<&'h str, &'h str>>,
    breakpoint: Pixel,
    font_families: Map<&'h str, &'h str>,
    used_font_families: Set<String>,
    media_queries: Map<String, Size>,
    styles: Set<String>,
    lang: Option<String>,
    generator: AtomicU16,
}

impl<'h> Header<'h> {
    pub fn new(head: &'h Option<MjHead>) -> Self {
        Self {
            head,
            attributes_all: head
                .as_ref()
                .map(|h| h.build_attributes_all())
                .unwrap_or_default(),
            attributes_class: head
                .as_ref()
                .map(|h| h.build_attributes_class())
                .unwrap_or_default(),
            attributes_element: head
                .as_ref()
                .map(|h| h.build_attributes_element())
                .unwrap_or_default(),
            breakpoint: head
                .as_ref()
                .and_then(|h| h.breakpoint())
                .and_then(|s| Pixel::try_from(s.value()).ok())
                .unwrap_or_else(|| Pixel::new(480.0)),
            font_families: head
                .as_ref()
                .map(|h| h.build_font_families())
                .unwrap_or_default(),
            used_font_families: Set::new(),
            media_queries: Map::new(),
            styles: Set::new(),
            lang: Default::default(),
            generator: AtomicU16::new(0),
        }
    }

    pub fn attribute_all(&self, key: &str) -> Option<&str> {
        self.attributes_all.get(key).copied()
    }

    pub fn attribute_class(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_class
            .get(name)
            .and_then(|class_map| class_map.get(key))
            .copied()
    }

    pub fn attribute_element(&self, name: &str, key: &str) -> Option<&str> {
        self.attributes_element
            .get(name)
            .and_then(|elt| elt.get(key))
            .copied()
    }

    pub fn head(&self) -> &Option<MjHead> {
        self.head
    }

    pub fn breakpoint(&self) -> &Pixel {
        &self.breakpoint
    }

    pub fn add_used_font_family(&mut self, value: &str) {
        self.used_font_families.insert(value.to_string());
    }

    pub fn add_font_families<T: AsRef<str>>(&mut self, value: T) {
        for name in value
            .as_ref()
            .split(',')
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
        {
            self.add_used_font_family(name);
        }
    }

    pub fn maybe_add_font_families<T: AsRef<str>>(&mut self, value: Option<T>) {
        if let Some(value) = value {
            self.add_font_families(value);
        }
    }

    pub fn used_font_families(&self) -> &Set<String> {
        &self.used_font_families
    }

    pub fn font_families(&self) -> &Map<&str, &str> {
        &self.font_families
    }

    pub fn media_queries(&self) -> &Map<String, Size> {
        &self.media_queries
    }

    pub fn add_media_query(&mut self, classname: String, size: Size) {
        self.media_queries.insert(classname, size);
    }

    pub fn styles(&self) -> &Set<String> {
        &self.styles
    }

    pub fn add_style(&mut self, value: String) {
        self.styles.insert(value);
    }

    pub fn maybe_add_style(&mut self, value: Option<String>) {
        if let Some(value) = value {
            self.add_style(value);
        }
    }

    pub fn lang(&self) -> Option<&str> {
        self.lang.as_deref()
    }

    pub fn maybe_set_lang(&mut self, value: Option<String>) {
        self.lang = value;
    }

    pub fn next_id(&self) -> String {
        let id = self.generator.fetch_add(1, Ordering::SeqCst);
        format!("{id:0>8}")
    }
}

pub trait Render<'header> {
    fn header(&self) -> Ref<Header<'header>>;
    fn tag(&self) -> Option<&str> {
        None
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        None
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
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

    fn default_attribute(&self, _key: &str) -> Option<&str> {
        None
    }

    fn attribute(&self, key: &str) -> Option<String> {
        if let Some(value) = self.attributes().and_then(|attrs| attrs.get(key)) {
            return Some(value.clone());
        }
        if let Some(value) = self.extra_attributes().and_then(|attrs| attrs.get(key)) {
            return Some(value.clone());
        }
        let header = self.header();
        if let Some(value) = self
            .attributes()
            .and_then(|attrs| attrs.get("mj-class"))
            .and_then(|mj_classes| {
                mj_classes
                    .split(' ')
                    .map(|mj_class| mj_class.trim())
                    .filter_map(|mj_class| header.attribute_class(mj_class, key))
                    .next()
            })
        {
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

    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
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

    fn render_fragment(&self, name: &str, opts: &RenderOptions) -> Result<String, Error> {
        match name {
            "main" => self.render(opts),
            _ => Err(Error::UnknownFragment(name.to_string())),
        }
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error>;
}

pub trait Renderable<'render, 'element: 'render, 'header: 'render> {
    fn is_raw(&'element self) -> bool {
        false
    }

    fn renderer(
        &'element self,
        header: Rc<RefCell<Header<'header>>>,
    ) -> Box<dyn Render<'header> + 'render>;
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
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn header_should_increase() {
        let head = None;
        let header = Rc::new(RefCell::new(super::Header::new(&head)));
        assert_eq!(header.borrow().next_id(), "00000000");
        assert_eq!(header.borrow().next_id(), "00000001");
        assert_eq!(header.borrow().next_id(), "00000002");
    }
}
