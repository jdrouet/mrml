use super::BodyElement;
use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::condition::*;
use crate::util::prelude::*;
use crate::util::{Context, Header, Style, Tag};
use crate::Options;
use log::debug;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJText {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJText {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJText, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(child, opts, None)?);
        }
        Ok(MJText {
            attributes: get_node_attributes(&node),
            context: None,
            children,
        })
    }
    fn get_style_text(&self) -> Style {
        let mut res = Style::new();
        res.maybe_set("font-family", self.get_attribute("font-family"));
        res.maybe_set("font-size", self.get_attribute("font-size"));
        res.maybe_set("font-style", self.get_attribute("font-style"));
        res.maybe_set("font-weight", self.get_attribute("font-weight"));
        res.maybe_set("letter-spacing", self.get_attribute("letter-spacing"));
        res.maybe_set("line-height", self.get_attribute("line-height"));
        res.maybe_set("text-align", self.get_attribute("align"));
        res.maybe_set("text-decoration", self.get_attribute("text-decoration"));
        res.maybe_set("text-transform", self.get_attribute("text-transform"));
        res.maybe_set("color", self.get_attribute("color"));
        res.maybe_set("height", self.get_attribute("height"));
        res
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        Ok(Tag::div()
            .insert_style(self.get_style_text().inner())
            .render(res.join("")))
    }

    fn render_with_height(&self, header: &Header, height: String) -> Result<String, Error> {
        let table = Tag::table();
        let tr = Tag::tr();
        let td = Tag::td()
            .set_attribute("height", &height)
            .set_style("height", &height)
            .set_style("vertical-align", "top");
        Ok(conditional_tag(table.render(
            tr.render(td.render(self.render_content(header)?)),
        )))
    }
}

impl Component for MJText {
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
        match self.get_attribute("height") {
            Some(value) => self.render_with_height(header, value),
            None => self.render_content(header),
        }
    }
}

impl ComponentWithAttributes for MJText {
    fn default_attribute(&self, key: &str) -> Option<String> {
        debug!("default_attribute {}", key);
        match key {
            "align" => Some("left".into()),
            "color" => Some("#000000".into()),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif".into()),
            "font-size" => Some("13px".into()),
            "line-height" => Some("1".into()),
            "padding" => Some("10px 25px".into()),
            _ => None,
        }
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.attributes)
    }
}

impl BodyComponent for MJText {
    fn get_style(&self, name: &str) -> Style {
        match name {
            "text" => self.get_style_text(),
            _ => Style::new(),
        }
    }
}

impl BodyContainedComponent for MJText {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-text.mjml"),
            include_str!("../../../test/mj-text.html"),
        );
    }

    #[test]
    fn doc_example() {
        compare_render(
            include_str!("../../../test/mj-text-example.mjml"),
            include_str!("../../../test/mj-text-example.html"),
        );
    }

    #[test]
    fn with_color() {
        compare_render(
            include_str!("../../../test/mj-text-color.mjml"),
            include_str!("../../../test/mj-text-color.html"),
        );
    }

    #[test]
    fn with_font_family() {
        compare_render(
            include_str!("../../../test/mj-text-font-family.mjml"),
            include_str!("../../../test/mj-text-font-family.html"),
        );
    }

    #[test]
    fn with_font_size() {
        compare_render(
            include_str!("../../../test/mj-text-font-size.mjml"),
            include_str!("../../../test/mj-text-font-size.html"),
        );
    }

    #[test]
    fn with_font_style() {
        compare_render(
            include_str!("../../../test/mj-text-font-style.mjml"),
            include_str!("../../../test/mj-text-font-style.html"),
        );
    }

    #[test]
    fn with_line_height() {
        compare_render(
            include_str!("../../../test/mj-text-line-height.mjml"),
            include_str!("../../../test/mj-text-line-height.html"),
        );
    }

    #[test]
    fn with_letter_spacing() {
        compare_render(
            include_str!("../../../test/mj-text-letter-spacing.mjml"),
            include_str!("../../../test/mj-text-letter-spacing.html"),
        );
    }

    #[test]
    fn with_height() {
        compare_render(
            include_str!("../../../test/mj-text-height.mjml"),
            include_str!("../../../test/mj-text-height.html"),
        );
    }

    #[test]
    fn with_decoration() {
        compare_render(
            include_str!("../../../test/mj-text-decoration.mjml"),
            include_str!("../../../test/mj-text-decoration.html"),
        );
    }

    #[test]
    fn with_transform() {
        compare_render(
            include_str!("../../../test/mj-text-transform.mjml"),
            include_str!("../../../test/mj-text-transform.html"),
        );
    }

    #[test]
    fn with_align() {
        compare_render(
            include_str!("../../../test/mj-text-align.mjml"),
            include_str!("../../../test/mj-text-align.html"),
        );
    }

    #[test]
    fn with_container_background_color() {
        compare_render(
            include_str!("../../../test/mj-text-container-background-color.mjml"),
            include_str!("../../../test/mj-text-container-background-color.html"),
        );
    }

    #[test]
    fn with_padding() {
        compare_render(
            include_str!("../../../test/mj-text-padding.mjml"),
            include_str!("../../../test/mj-text-padding.html"),
        );
    }

    #[test]
    fn with_css_class() {
        compare_render(
            include_str!("../../../test/mj-text-class.mjml"),
            include_str!("../../../test/mj-text-class.html"),
        );
    }
}
