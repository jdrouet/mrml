use super::error::Error;
use super::prelude::{Component, ContainedComponent};
// use super::Element;
// use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Spacing, Style};
use crate::{close_tag, closed_tag, open_tag, with_tag};
use log::debug;
use roxmltree::Node;

const ALLOWED_ATTRIBUTES: [&'static str; 28] = [
    "align",
    "alt",
    "href",
    "name",
    "src",
    "srcset",
    "title",
    "rel",
    "enum",
    "border",
    "border-top",
    "border-right",
    "border-bottom",
    "border-left",
    "border-radius",
    "container-background-color",
    "fluid-on-mobile",
    "padding",
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
    "target",
    "width",
    "height",
    "max-height",
    "font-size",
    "usemap",
];

#[derive(Clone, Debug)]
pub struct MJImage<'a, 'b> {
    context: Option<Context>,
    node: Node<'a, 'b>,
}

impl MJImage<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJImage<'a, 'b>, Error> {
        Ok(MJImage {
            context: None,
            node,
        })
    }

    fn get_padding_left(&self) -> Size {
        match self
            .get_attribute("padding-left")
            .and_then(|value| value.parse::<Size>().ok())
            .or_else(|| {
                self.get_attribute("padding")
                    .and_then(|value| value.parse::<Spacing>().ok())
                    .and_then(|value| Some(value.left))
            }) {
            Some(value) => value,
            None => Size::Pixel(0.0),
        }
    }

    fn get_padding_right(&self) -> Size {
        match self
            .get_attribute("padding-right")
            .and_then(|value| value.parse::<Size>().ok())
            .or_else(|| {
                self.get_attribute("padding")
                    .and_then(|value| value.parse::<Spacing>().ok())
                    .and_then(|value| Some(value.right))
            }) {
            Some(value) => value,
            None => Size::Pixel(0.0),
        }
    }

    fn get_padding_horizontal(&self) -> Size {
        let left = self.get_padding_left();
        let right = self.get_padding_right();
        if left.is_pixel() && right.is_pixel() {
            Size::Pixel(left.value() + right.value())
        } else {
            Size::Pixel(0.0)
        }
    }

    fn get_border_horizontal(&self) -> Size {
        // TODO
        Size::Pixel(0.0)
    }

    fn get_box_widths(&self) -> Size {
        let container_width = self
            .get_container_width()
            .or(Some(Size::Pixel(0.0)))
            .unwrap();
        let paddings = self.get_padding_horizontal();
        let borders = self.get_border_horizontal();
        Size::Pixel(container_width.value() - paddings.value() - borders.value())
    }

    fn get_content_width(&self) -> Option<Size> {
        self.get_attribute("width")
            .and_then(|width| width.parse::<Size>().ok())
            .and_then(|width| {
                let box_size = self.get_box_widths();
                if width.value() < box_size.value() {
                    Some(width)
                } else {
                    Some(box_size)
                }
            })
    }

    fn render_image(&self) -> String {
        let mut attrs = Attributes::new();
        attrs.maybe_set("alt", self.get_attribute("alt"));
        match self
            .get_attribute("height")
            .and_then(|height| height.parse::<Size>().ok())
        {
            Some(height) => {
                attrs.set("height", height);
            }
            None => {
                attrs.set("height", "auto");
            }
        };
        // TODO height
        attrs.maybe_set("src", self.get_attribute("src"));
        attrs.maybe_set("srcset", self.get_attribute("srcset"));
        attrs.set("style", self.get_style("img").to_string());
        attrs.maybe_set("title", self.get_attribute("title"));
        attrs.maybe_set(
            "width",
            self.get_content_width()
                .and_then(|width| Some(width.value())),
        );
        attrs.maybe_set("usemap", self.get_attribute("usemap"));
        closed_tag!("img", attrs.to_string())
    }

    fn render_link(&self) -> String {
        let mut attrs = Attributes::new();
        attrs.maybe_set("href", self.get_attribute("href"));
        attrs.maybe_set("target", self.get_attribute("target"));
        attrs.maybe_set("rel", self.get_attribute("rel"));
        attrs.maybe_set("name", self.get_attribute("name"));
        with_tag!("a", attrs.to_string(), self.render_image())
    }
}

impl Component for MJImage<'_, '_> {
    fn allowed_attributes() -> Option<Vec<&'static str>> {
        Some(ALLOWED_ATTRIBUTES.to_vec())
    }

    fn default_attribute(key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("center".into()),
            "border" => Some("0".into()),
            "height" => Some("auto".into()),
            "padding" => Some("10px 25px".into()),
            "target" => Some("_blank".into()),
            "font-size" => Some("13px".into()),
            _ => None,
        }
    }

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        if let Some(ctx) = self.context() {
            let mut style = format!(
                "@media only screen and (max-width:{}) {{\n",
                ctx.options().breakpoint.to_string(),
            );
            style.push_str("table.mj-full-width-mobile { width: 100% !important; }\n");
            style.push_str("td.mj-full-width-mobile { width: auto !important; }\n");
            style.push_str("}\n");
            header.add_style(style);
        }
        header
    }

    fn get_style(&self, name: &str) -> Style {
        let mut res = Style::new();
        let full_width = self.get_attribute("full-width").is_some();
        match name {
            "img" => {
                res.maybe_set("border", self.get_attribute("border"));
                res.maybe_set("border-left", self.get_attribute("left"));
                res.maybe_set("border-right", self.get_attribute("right"));
                res.maybe_set("border-top", self.get_attribute("top"));
                res.maybe_set("border-bottom", self.get_attribute("bottom"));
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("display", "block");
                res.set("outline", "none");
                res.set("text-decoration", "none");
                res.maybe_set("height", self.get_attribute("height"));
                res.maybe_set("max-height", self.get_attribute("max-height"));
                res.set("width", "100%");
                if full_width {
                    res.set("min-width", "100%");
                    res.set("max-width", "100%");
                }
                res.maybe_set("font-size", self.get_attribute("font-size"));
            }
            "td" => {
                if !full_width {
                    res.maybe_set("width", self.get_content_width());
                }
            }
            "table" => {
                if full_width {
                    res.set("min-width", "100%");
                    res.set("max-width", "100%");
                    res.maybe_set("width", self.get_content_width());
                }
                res.set("border-collapse", "collapse");
                res.set("border-spacing", "0px");
            }
            _ => (),
        };
        res
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn node(&self) -> Option<Node> {
        Some(self.node)
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self) -> Result<String, Error> {
        let mut res = vec![];
        {
            let mut attrs = Attributes::new();
            attrs.set("border", "0");
            attrs.set("cellpadding", "0");
            attrs.set("cellspacing", "0");
            attrs.set("role", "presentation");
            attrs.set("style", self.get_style("table").to_string());
            if self.get_attribute("fluid-on-mobile").is_some() {
                attrs.set("class", "mj-full-width-mobile");
            }
            res.push(open_tag!("table", attrs.to_string()));
        };
        res.push(open_tag!("tbody"));
        res.push(open_tag!("tr"));
        {
            let mut attrs = Attributes::new();
            attrs.set("style", self.get_style("td").to_string());
            if self.get_attribute("fluid-on-mobile").is_some() {
                attrs.set("class", "mj-full-width-mobile");
            }
            res.push(open_tag!("td", attrs.to_string()));
        };
        if self.get_attribute("href").is_some() {
            res.push(self.render_link());
        } else {
            res.push(self.render_image());
        }
        // TODO render image
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("tbody"));
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }
}

impl ContainedComponent for MJImage<'_, '_> {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../test/mj-image.mjml"),
            include_str!("../../test/mj-image.html"),
        );
    }
}
