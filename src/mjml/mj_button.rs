use super::error::Error;
use super::prelude::*;
use super::Element;
use crate::util::prelude::PropertyMap;
use crate::util::{Attributes, Context, Header, Size, Style};
use crate::{close_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJButton {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<Element>,
}

impl MJButton {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJButton, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(MJButton {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    fn get_content(&self) -> Result<String, Error> {
        let mut res = String::from("");
        for item in self.children.iter() {
            res.push_str(item.render()?.as_str());
        }
        Ok(res)
    }

    fn calculate_a_width(&self, width: Option<Size>) -> Option<Size> {
        let width = match width {
            Some(value) => value,
            None => return None,
        };
        if !width.is_pixel() {
            return None;
        }
        let pad_left = match self.get_prefixed_padding_left("inner") {
            Some(value) => value.value(),
            None => 0.0,
        };
        let pad_right = match self.get_prefixed_padding_right("inner") {
            Some(value) => value.value(),
            None => 0.0,
        };

        Some(Size::Pixel(width.value() + pad_left + pad_right))
    }
}

impl Component for MJButton {
    fn allowed_attributes(&self) -> Option<Vec<&'static str>> {
        None
    }

    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("center".into()),
            "background-color" => Some("#414141".into()),
            "border" => Some("none".into()),
            "border-radius" => Some("3px".into()),
            "color" => Some("#ffffff".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "font-size" => Some("13px".into()),
            "font-weight" => Some("normal".into()),
            "inner-padding" => Some("10px 25px".into()),
            "line-height" => Some("120%".into()),
            "padding" => Some("10px 25px".into()),
            "target" => Some("_blank".into()),
            "text-decoration" => Some("none".into()),
            "text-transform" => Some("none".into()),
            "vertical-align" => Some("middle".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        header.maybe_add_font_families(self.get_attribute("font-family"));
        header
    }

    fn get_style(&self, name: &str) -> Style {
        let mut res = Style::new();
        match name {
            "table" => {
                res.set("border-collapse", "separate");
                res.maybe_set("width", self.get_attribute("width"));
                res.set("line-height", "100%");
            }
            "td" => {
                res.maybe_set("background", self.get_attribute("background-color"));
                res.maybe_set("border", self.get_attribute("border"));
                res.maybe_set("border-top", self.get_attribute("border-top"));
                res.maybe_set("border-right", self.get_attribute("border-right"));
                res.maybe_set("border-bottom", self.get_attribute("border-bottom"));
                res.maybe_set("border-left", self.get_attribute("border-left"));
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
                res.set("cursor", "auto");
                res.maybe_set("font-style", self.get_attribute("font-style"));
                res.maybe_set("height", self.get_attribute("height"));
                res.maybe_set("mso-padding-alt", self.get_attribute("inner-padding"));
                res.maybe_set("text-align", self.get_attribute("text-align"));
            }
            "content" => {
                res.set("display", "inline-block");
                res.maybe_set(
                    "width",
                    self.get_size_attribute("width")
                        .and_then(|value| self.calculate_a_width(Some(value))),
                );
                res.maybe_set("background", self.get_attribute("background-color"));
                res.maybe_set("color", self.get_attribute("color"));
                res.maybe_set("font-family", self.get_attribute("font-family"));
                res.maybe_set("font-size", self.get_attribute("font-size"));
                res.maybe_set("font-style", self.get_attribute("font-style"));
                res.maybe_set("font-weight", self.get_attribute("font-weight"));
                res.maybe_set("line-height", self.get_attribute("line-height"));
                res.maybe_set("line-spacing", self.get_attribute("line-spacing"));
                res.set("margin", "0");
                res.maybe_set("text-decoration", self.get_attribute("text-decoration"));
                res.maybe_set("text-transform", self.get_attribute("text-transform"));
                res.maybe_set("padding", self.get_attribute("inner-padding"));
                res.set("mso-padding-alt", "0px");
                res.maybe_set("border-radius", self.get_attribute("border-radius"));
            }
            _ => (),
        };
        res
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self) -> Result<String, Error> {
        let mut res = vec![];
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0"),
                ("role", "presentation"),
                ("style", self.get_style("table").to_string())
            )
        ));
        res.push(open_tag!("tr"));
        {
            let mut attrs = Attributes::new();
            attrs.set("align", "center");
            attrs.maybe_set("bgcolor", self.get_attribute("background-color"));
            attrs.set("role", "presentation");
            attrs.set("style", self.get_style("td"));
            attrs.maybe_set("valign", self.get_attribute("vertical-align"));
            res.push(open_tag!("td", attrs.to_string()));
        }
        let tag_name = match self.get_attribute("href") {
            Some(_) => "a",
            None => "p",
        };
        let mut attrs = Attributes::new();
        attrs.maybe_set("href", self.get_attribute("href"));
        attrs.maybe_set("rel", self.get_attribute("rel"));
        attrs.maybe_set("name", self.get_attribute("name"));
        attrs.set("style", self.get_style("content"));
        if self.get_attribute("href").is_some() {
            attrs.maybe_set("target", self.get_attribute("target"));
        }
        res.push(open_tag!(tag_name, attrs.to_string()));
        res.push(self.get_content()?);
        res.push(close_tag!(tag_name));
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
        Ok(res.join(""))
    }
}

impl ContainedComponent for MJButton {}
impl ComponentWithSizeAttribute for MJButton {}
impl ComponentWithPadding for MJButton {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../test/mj-button.mjml"),
            include_str!("../../test/mj-button.html"),
        );
    }

    #[test]
    fn example() {
        compare_render(
            include_str!("../../test/mj-button-example.mjml"),
            include_str!("../../test/mj-button-example.html"),
        );
    }

    #[test]
    fn with_align() {
        compare_render(
            include_str!("../../test/mj-button-align.mjml"),
            include_str!("../../test/mj-button-align.html"),
        );
    }

    #[test]
    fn with_background() {
        compare_render(
            include_str!("../../test/mj-button-background.mjml"),
            include_str!("../../test/mj-button-background.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../test/mj-button-border.mjml"),
            include_str!("../../test/mj-button-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../test/mj-button-border-radius.mjml"),
            include_str!("../../test/mj-button-border-radius.html"),
        );
    }

    #[test]
    fn with_color() {
        compare_render(
            include_str!("../../test/mj-button-color.mjml"),
            include_str!("../../test/mj-button-color.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../test/mj-button-container-background-color.mjml"),
            include_str!("../../test/mj-button-container-background-color.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../test/mj-button-class.mjml"),
            include_str!("../../test/mj-button-class.html"),
        );
    }

    #[test]
    fn with_font_family() {
        compare_render(
            include_str!("../../test/mj-button-font-family.mjml"),
            include_str!("../../test/mj-button-font-family.html"),
        );
    }

    #[test]
    fn with_font_size() {
        compare_render(
            include_str!("../../test/mj-button-font-size.mjml"),
            include_str!("../../test/mj-button-font-size.html"),
        );
    }

    #[test]
    fn with_font_style() {
        compare_render(
            include_str!("../../test/mj-button-font-style.mjml"),
            include_str!("../../test/mj-button-font-style.html"),
        );
    }

    #[test]
    fn with_font_weight() {
        compare_render(
            include_str!("../../test/mj-button-font-weight.mjml"),
            include_str!("../../test/mj-button-font-weight.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../test/mj-button-height.mjml"),
            include_str!("../../test/mj-button-height.html"),
        );
    }

    #[test]
    fn with_href() {
        compare_render(
            include_str!("../../test/mj-button-href.mjml"),
            include_str!("../../test/mj-button-href.html"),
        );
    }

    #[test]
    fn with_inner_padding() {
        compare_render(
            include_str!("../../test/mj-button-inner-padding.mjml"),
            include_str!("../../test/mj-button-inner-padding.html"),
        );
    }

    #[test]
    fn with_line_height() {
        compare_render(
            include_str!("../../test/mj-button-line-height.mjml"),
            include_str!("../../test/mj-button-line-height.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../test/mj-button-padding.mjml"),
            include_str!("../../test/mj-button-padding.html"),
        );
    }

    #[test]
    fn with_text_decoration() {
        compare_render(
            include_str!("../../test/mj-button-text-decoration.mjml"),
            include_str!("../../test/mj-button-text-decoration.html"),
        );
    }
}
