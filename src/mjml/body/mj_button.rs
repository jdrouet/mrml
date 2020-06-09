use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::{Context, Header, Size, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJButton {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJButton {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJButton, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(&child, opts, None)?);
        }
        Ok(MJButton {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }

    fn get_content(&self, header: &Header) -> Result<String, Error> {
        let mut res = String::from("");
        for item in self.children.iter() {
            res.push_str(item.render(header)?.as_str());
        }
        Ok(res)
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.set_style("border-collapse", "separate")
            .maybe_set_style("width", self.get_attribute("width"))
            .set_style("line-height", "100%")
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("border-top", self.get_attribute("border-top"))
            .maybe_set_style("border-right", self.get_attribute("border-right"))
            .maybe_set_style("border-bottom", self.get_attribute("border-bottom"))
            .maybe_set_style("border-left", self.get_attribute("border-left"))
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
            .set_style("cursor", "auto")
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("height", self.get_attribute("height"))
            .maybe_set_style("mso-padding-alt", self.get_attribute("inner-padding"))
            .maybe_set_style("text-align", self.get_attribute("text-align"))
    }

    fn set_style_content(&self, tag: Tag) -> Tag {
        tag.set_style("display", "inline-block")
            .maybe_set_style(
                "width",
                self.get_size_attribute("width")
                    .and_then(|value| self.calculate_a_width(Some(value))),
            )
            .maybe_set_style("background", self.get_attribute("background-color"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("line-spacing", self.get_attribute("line-spacing"))
            .set_style("margin", "0")
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
            .maybe_set_style("text-transform", self.get_attribute("text-transform"))
            .maybe_set_style("padding", self.get_attribute("inner-padding"))
            .set_style("mso-padding-alt", "0px")
            .maybe_set_style("border-radius", self.get_attribute("border-radius"))
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

        Some(Size::Pixel(width.value() - pad_left - pad_right))
    }
}

impl Component for MJButton {
    fn update_header(&self, header: &mut Header) {
        header.maybe_add_font_families(self.get_attribute("font-family"));
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let table = self.set_style_table(Tag::table_presentation());
        let tr = Tag::tr();
        let td = self
            .set_style_td(Tag::td())
            .set_attribute("align", "center")
            .maybe_set_attribute("bgcolor", self.get_attribute("background-color"))
            .set_attribute("role", "presentation")
            .maybe_set_attribute("valign", self.get_attribute("vertical-align"));
        let link = Tag::new(match self.get_attribute("href") {
            Some(_) => "a",
            None => "p",
        })
        .maybe_set_attribute("href", self.get_attribute("href"))
        .maybe_set_attribute("rel", self.get_attribute("rel"))
        .maybe_set_attribute("name", self.get_attribute("name"))
        .maybe_set_attribute(
            "target",
            self.get_attribute("href")
                .and_then(|_v| self.get_attribute("target")),
        );
        let link = self.set_style_content(link);

        Ok(table.render(tr.render(td.render(link.render(self.get_content(header)?)))))
    }
}

impl ComponentWithAttributes for MJButton {
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
}

impl BodyComponent for MJButton {
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "table" => self.set_style_table(tag),
            "td" => self.set_style_td(tag),
            "content" => self.set_style_content(tag),
            _ => tag,
        }
    }
}

impl ComponentWithSizeAttribute for MJButton {}
impl BodyComponentWithPadding for MJButton {}
impl BodyContainedComponent for MJButton {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-button.mjml"),
            include_str!("../../../test/mj-button.html"),
        );
    }

    #[test]
    fn example() {
        compare_render(
            include_str!("../../../test/mj-button-example.mjml"),
            include_str!("../../../test/mj-button-example.html"),
        );
    }

    #[test]
    fn with_align() {
        compare_render(
            include_str!("../../../test/mj-button-align.mjml"),
            include_str!("../../../test/mj-button-align.html"),
        );
    }

    #[test]
    fn with_background() {
        compare_render(
            include_str!("../../../test/mj-button-background.mjml"),
            include_str!("../../../test/mj-button-background.html"),
        );
    }

    #[test]
    fn with_border() {
        compare_render(
            include_str!("../../../test/mj-button-border.mjml"),
            include_str!("../../../test/mj-button-border.html"),
        );
    }

    #[test]
    fn with_border_radius() {
        compare_render(
            include_str!("../../../test/mj-button-border-radius.mjml"),
            include_str!("../../../test/mj-button-border-radius.html"),
        );
    }

    #[test]
    fn with_color() {
        compare_render(
            include_str!("../../../test/mj-button-color.mjml"),
            include_str!("../../../test/mj-button-color.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../../test/mj-button-container-background-color.mjml"),
            include_str!("../../../test/mj-button-container-background-color.html"),
        );
    }

    #[test]
    fn with_class() {
        compare_render(
            include_str!("../../../test/mj-button-class.mjml"),
            include_str!("../../../test/mj-button-class.html"),
        );
    }

    #[test]
    fn with_font_family() {
        compare_render(
            include_str!("../../../test/mj-button-font-family.mjml"),
            include_str!("../../../test/mj-button-font-family.html"),
        );
    }

    #[test]
    fn with_font_size() {
        compare_render(
            include_str!("../../../test/mj-button-font-size.mjml"),
            include_str!("../../../test/mj-button-font-size.html"),
        );
    }

    #[test]
    fn with_font_style() {
        compare_render(
            include_str!("../../../test/mj-button-font-style.mjml"),
            include_str!("../../../test/mj-button-font-style.html"),
        );
    }

    #[test]
    fn with_font_weight() {
        compare_render(
            include_str!("../../../test/mj-button-font-weight.mjml"),
            include_str!("../../../test/mj-button-font-weight.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../../test/mj-button-height.mjml"),
            include_str!("../../../test/mj-button-height.html"),
        );
    }

    #[test]
    fn with_href() {
        compare_render(
            include_str!("../../../test/mj-button-href.mjml"),
            include_str!("../../../test/mj-button-href.html"),
        );
    }

    #[test]
    fn with_inner_padding() {
        compare_render(
            include_str!("../../../test/mj-button-inner-padding.mjml"),
            include_str!("../../../test/mj-button-inner-padding.html"),
        );
    }

    #[test]
    fn with_line_height() {
        compare_render(
            include_str!("../../../test/mj-button-line-height.mjml"),
            include_str!("../../../test/mj-button-line-height.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-button-padding.mjml"),
            include_str!("../../../test/mj-button-padding.html"),
        );
    }

    #[test]
    fn with_text_decoration() {
        compare_render(
            include_str!("../../../test/mj-button-text-decoration.mjml"),
            include_str!("../../../test/mj-button-text-decoration.html"),
        );
    }

    #[test]
    fn with_text_transform() {
        compare_render(
            include_str!("../../../test/mj-button-text-transform.mjml"),
            include_str!("../../../test/mj-button-text-transform.html"),
        );
    }

    #[test]
    fn with_vertical_align() {
        compare_render(
            include_str!("../../../test/mj-button-vertical-align.mjml"),
            include_str!("../../../test/mj-button-vertical-align.html"),
        );
    }

    #[test]
    fn with_width() {
        compare_render(
            include_str!("../../../test/mj-button-width.mjml"),
            include_str!("../../../test/mj-button-width.html"),
        );
    }
}
