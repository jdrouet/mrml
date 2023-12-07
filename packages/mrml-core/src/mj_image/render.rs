use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjImage, NAME};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjImageRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjImage,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MjImageRender<'e, 'h> {
    fn is_fluid_on_mobile(&self) -> bool {
        self.attribute("fluid-on-mobile")
            .and_then(|value| value.parse::<bool>().ok())
            .unwrap_or(false)
    }

    fn is_full_width(&self) -> bool {
        self.attribute_exists("full-width")
    }

    fn get_box_width(&self) -> Option<Pixel> {
        self.container_width.as_ref().map(|width| {
            let hborder = self.get_border_horizontal();
            let hpadding = self.get_padding_horizontal();
            Pixel::new(width.value() - hborder.value() - hpadding.value())
        })
    }

    fn get_content_width(&self) -> Option<Pixel> {
        self.attribute_as_pixel("width")
            .map(|width| match self.get_box_width() {
                Some(box_size) => {
                    if width.value() < box_size.value() {
                        width
                    } else {
                        box_size
                    }
                }
                None => width,
            })
            // when no width given
            .or_else(|| self.get_box_width())
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        let tag = tag
            .maybe_add_style("border", self.attribute("border"))
            .maybe_add_style("border-left", self.attribute("left"))
            .maybe_add_style("border-right", self.attribute("right"))
            .maybe_add_style("border-top", self.attribute("top"))
            .maybe_add_style("border-bottom", self.attribute("bottom"))
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("display", "block")
            .add_style("outline", "none")
            .add_style("text-decoration", "none")
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("max-height", self.attribute("max-height"))
            .add_style("width", "100%");
        let tag = if self.is_full_width() {
            tag.add_style("min-width", "100%")
                .add_style("max-width", "100%")
        } else {
            tag
        };
        tag.maybe_add_style("font-size", self.attribute("font-size"))
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        if self.is_full_width() {
            tag
        } else {
            tag.maybe_add_style("width", self.get_content_width().map(|v| v.to_string()))
        }
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        let tag = if self.is_full_width() {
            tag.add_style("min-width", "100%")
                .add_style("max-width", "100%")
                .maybe_add_style("width", self.get_content_width().map(|v| v.to_string()))
        } else {
            tag
        };
        tag.add_style("border-collapse", "collapse")
            .add_style("border-spacing", "0px")
    }

    fn render_image(&self) -> String {
        let img = Tag::new("img")
            .maybe_add_attribute("alt", self.attribute("alt"))
            .add_attribute(
                "height",
                self.attribute_as_size("height")
                    .map(|size| size.value().to_string())
                    .unwrap_or_else(|| "auto".into()),
            )
            .maybe_add_attribute("src", self.attribute("src"))
            .maybe_add_attribute("srcset", self.attribute("srcset"))
            .maybe_add_attribute("title", self.attribute("title"))
            .maybe_add_attribute(
                "width",
                self.get_content_width()
                    .map(|size| size.value().to_string()),
            )
            .maybe_add_attribute("usemap", self.attribute("usemap"));
        self.set_style_img(img).closed()
    }

    fn render_link(&self) -> String {
        Tag::new("a")
            .maybe_add_attribute("href", self.attribute("href"))
            .maybe_add_attribute("name", self.attribute("name"))
            .maybe_add_attribute("rel", self.attribute("rel"))
            .maybe_add_attribute("target", self.attribute("target"))
            .render(self.render_image())
    }

    fn render_style(&self) -> String {
        format!(
            r#"@media only screen and (max-width:{}) {{
                table.mj-full-width-mobile {{ width: 100% !important; }}
                td.mj-full-width-mobile {{ width: auto !important; }}
            }}
            "#,
            self.header.borrow().breakpoint().lower().to_string(),
        )
    }
}

impl<'e, 'h> Render<'h> for MjImageRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "align" => Some("center"),
            "border" => Some("0"),
            "height" => Some("auto"),
            "padding" => Some("10px 25px"),
            "target" => Some("_blank"),
            "font-size" => Some("13px"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, _opts: &RenderOptions) -> Result<String, Error> {
        let style = self.render_style();
        self.header.borrow_mut().add_style(style);
        let class = if self.is_fluid_on_mobile() {
            Some("mj-full-width-mobile")
        } else {
            None
        };
        let table = self
            .set_style_table(Tag::table_presentation())
            .maybe_add_class(class);
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_td(Tag::td()).maybe_add_class(class);
        let content = if self.attribute_exists("href") {
            self.render_link()
        } else {
            self.render_image()
        };
        Ok(table.render(tbody.render(tr.render(td.render(content)))))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjImage {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjImageRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-image");
    crate::should_render!(align, "mj-image-align");
    crate::should_render!(border_radius, "mj-image-border-radius");
    crate::should_render!(border, "mj-image-border");
    crate::should_render!(class, "mj-image-class");
    crate::should_render!(
        container_background_color,
        "mj-image-container-background-color"
    );
    crate::should_render!(height, "mj-image-height");
    crate::should_render!(href, "mj-image-href");
    crate::should_render!(padding, "mj-image-padding");
}
