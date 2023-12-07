use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjButton, NAME};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjButtonRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjButton,
}

impl<'e, 'h> MjButtonRender<'e, 'h> {
    fn content_width(&self) -> Option<String> {
        if let Some(width) = self.attribute_as_pixel("width") {
            let pad_left = self
                .attribute_as_pixel("inner-padding-left")
                .map(|pad| pad.value())
                .or_else(|| {
                    self.attribute_as_spacing("inner-padding")
                        .map(|pad| pad.left().value())
                })
                .unwrap_or(0.0);
            let pad_right = self
                .attribute_as_pixel("inner-padding-right")
                .map(|pad| pad.value())
                .or_else(|| {
                    self.attribute_as_spacing("inner-padding")
                        .map(|pad| pad.right().value())
                })
                .unwrap_or(0.0);
            Some(Pixel::new(width.value() - pad_left - pad_right).to_string())
        } else {
            None
        }
    }

    fn render_children(&self, opts: &RenderOptions) -> Result<String, Error> {
        self.element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render(opts)?)
            })
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.add_style("border-collapse", "separate")
            .maybe_add_style("width", self.attribute("width"))
            .add_style("line-height", "100%")
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("border", self.attribute("border"))
            .maybe_add_style("border-bottom", self.attribute("border-bottom"))
            .maybe_add_style("border-left", self.attribute("border-left"))
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .maybe_add_style("border-right", self.attribute("border-right"))
            .maybe_add_style("border-top", self.attribute("border-top"))
            .add_style("cursor", "auto")
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("mso-padding-alt", self.attribute("inner-padding"))
            .maybe_add_style("text-align", self.attribute("text-align"))
            .maybe_add_style("background", self.attribute("background-color"))
    }

    fn set_style_content(&self, tag: Tag) -> Tag {
        tag.add_style("display", "inline-block")
            .maybe_add_style("width", self.content_width())
            .maybe_add_style("background", self.attribute("background-color"))
            .maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("line-spacing", self.attribute("line-spacing"))
            .add_style("margin", "0")
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
            .maybe_add_style("text-transform", self.attribute("text-transform"))
            .maybe_add_style("padding", self.attribute("inner-padding"))
            .add_style("mso-padding-alt", "0px")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
    }
}

impl<'e, 'h> Render<'h> for MjButtonRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "align" => Some("center"),
            "background-color" => Some("#414141"),
            "border" => Some("none"),
            "border-radius" => Some("3px"),
            "color" => Some("#ffffff"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "font-weight" => Some("normal"),
            "inner-padding" => Some("10px 25px"),
            "line-height" => Some("120%"),
            "padding" => Some("10px 25px"),
            "target" => Some("_blank"),
            "text-decoration" => Some("none"),
            "text-transform" => Some("none"),
            "vertical-align" => Some("middle"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let font_family = self.attribute("font-family");
        self.header
            .borrow_mut()
            .maybe_add_font_families(font_family);
        let table = self.set_style_table(Tag::table_presentation());
        let tr = Tag::tr();
        let td = self
            .set_style_td(Tag::td())
            .add_attribute("align", "center")
            .maybe_add_attribute("bgcolor", self.attribute("background-color"))
            .add_attribute("role", "presentation")
            .maybe_add_attribute("valign", self.attribute("vertical-align"));
        let link = Tag::new(self.attribute("href").map(|_| "a").unwrap_or("p"))
            .maybe_add_attribute("href", self.attribute("href"))
            .maybe_add_attribute("rel", self.attribute("rel"))
            .maybe_add_attribute("name", self.attribute("name"))
            .maybe_add_attribute(
                "target",
                self.attribute("href")
                    .and_then(|_v| self.attribute("target")),
            );
        let link = self.set_style_content(link);

        Ok(table.render(
            Tag::tbody().render(tr.render(td.render(link.render(self.render_children(opts)?)))),
        ))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjButton {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjButtonRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-button");
    crate::should_render!(align, "mj-button-align");
    crate::should_render!(background, "mj-button-background");
    crate::should_render!(border_radius, "mj-button-border-radius");
    crate::should_render!(border, "mj-button-border");
    crate::should_render!(class, "mj-button-class");
    crate::should_render!(color, "mj-button-color");
    crate::should_render!(
        container_background_color,
        "mj-button-container-background-color"
    );
    crate::should_render!(example, "mj-button-example");
    crate::should_render!(font_family, "mj-button-font-family");
    crate::should_render!(font_size, "mj-button-font-size");
    crate::should_render!(font_style, "mj-button-font-style");
    crate::should_render!(font_weight, "mj-button-font-weight");
    crate::should_render!(height, "mj-button-height");
    crate::should_render!(href, "mj-button-href");
    crate::should_render!(inner_padding, "mj-button-inner-padding");
    crate::should_render!(line_height, "mj-button-line-height");
    crate::should_render!(padding, "mj-button-padding");
    crate::should_render!(text_decoration, "mj-button-text-decoration");
    crate::should_render!(text_transform, "mj-button-text-transform");
    crate::should_render!(vertical_align, "mj-button-vertical-align");
    crate::should_render!(width, "mj-button-width");
}
