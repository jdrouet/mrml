use super::error::Error;
use super::prelude::{Component, ContainedComponent};
use super::Element;
use crate::util::condition::*;
use crate::util::prelude::PropertyMap;
use crate::util::{Context, Header, Style};
use crate::{close_tag, open_tag, to_attributes};
use log::debug;
use roxmltree::Node;

const ALLOWED_ATTRIBUTES: [&'static str; 19] = [
    "color",
    "align",
    "font-family",
    "font-size",
    "font-style",
    "font-weight",
    "line-height",
    "letter-spacing",
    "height",
    "text-decoration",
    "text-transform",
    "align",
    "container-background-color",
    "padding",
    "padding-top",
    "padding-bottom",
    "padding-left",
    "padding-right",
    "css-class",
];

#[derive(Clone, Debug)]
pub struct MJText<'a, 'b> {
    context: Option<Context>,
    node: Node<'a, 'b>,
    children: Vec<Element<'a, 'b>>,
}

impl MJText<'_, '_> {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>) -> Result<MJText<'a, 'b>, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(Element::parse(child)?);
        }
        Ok(MJText {
            context: None,
            node,
            children,
        })
    }

    fn render_content(&self) -> Result<String, Error> {
        let style = self.get_style("text");
        let mut res = vec![];
        res.push(open_tag!(
            "div",
            to_attributes!(("style", style.to_string()))
        ));
        for child in self.children.iter() {
            res.push(child.render()?);
        }
        res.push(close_tag!("div"));
        Ok(res.join(""))
    }

    fn render_with_height(&self, height: String) -> Result<String, Error> {
        let mut res = vec![];
        res.push(START_CONDITIONAL_TAG.into());
        res.push(open_tag!(
            "table",
            to_attributes!(
                ("role", "presentation"),
                ("border", "0"),
                ("cellpadding", "0"),
                ("cellspacing", "0")
            )
        ));
        res.push(open_tag!("tr"));
        res.push(open_tag!(
            "td",
            to_attributes!(
                ("height", height),
                ("style", format!("height:{};vertical-align:top;", height))
            )
        ));
        res.push(self.render_content()?);
        res.push(close_tag!("td"));
        res.push(close_tag!("tr"));
        res.push(close_tag!("table"));
        res.push(END_CONDITIONAL_TAG.into());
        Ok(res.join(""))
    }
}

impl Component for MJText<'_, '_> {
    fn allowed_attributes() -> Option<Vec<&'static str>> {
        Some(ALLOWED_ATTRIBUTES.to_vec())
    }

    fn default_attribute(key: &str) -> Option<String> {
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

    fn to_header(&self) -> Header {
        let mut header = Header::new();
        header.maybe_add_font_families(self.get_attribute("font-family"));
        header
    }

    fn get_style(&self, name: &str) -> Style {
        let mut res = Style::new();
        match name {
            "text" => {
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
        match self.get_attribute("height") {
            Some(value) => self.render_with_height(value),
            None => self.render_content(),
        }
    }
}

impl ContainedComponent for MJText<'_, '_> {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../test/mj-text.mjml"),
            include_str!("../../test/mj-text.html"),
        );
    }

    #[test]
    fn doc_example() {
        compare_render(
            include_str!("../../test/mj-text-example.mjml"),
            include_str!("../../test/mj-text-example.html"),
        );
    }

    #[test]
    fn with_color() {
        compare_render(
            include_str!("../../test/mj-text-color.mjml"),
            include_str!("../../test/mj-text-color.html"),
        );
    }

    #[test]
    fn with_font_family() {
        compare_render(
            include_str!("../../test/mj-text-font-family.mjml"),
            include_str!("../../test/mj-text-font-family.html"),
        );
    }

    #[test]
    fn with_font_size() {
        compare_render(
            include_str!("../../test/mj-text-font-size.mjml"),
            include_str!("../../test/mj-text-font-size.html"),
        );
    }

    #[test]
    fn with_font_style() {
        compare_render(
            include_str!("../../test/mj-text-font-style.mjml"),
            include_str!("../../test/mj-text-font-style.html"),
        );
    }

    #[test]
    fn with_line_height() {
        compare_render(
            include_str!("../../test/mj-text-line-height.mjml"),
            include_str!("../../test/mj-text-line-height.html"),
        );
    }

    #[test]
    fn with_letter_spacing() {
        compare_render(
            include_str!("../../test/mj-text-letter-spacing.mjml"),
            include_str!("../../test/mj-text-letter-spacing.html"),
        );
    }
}
