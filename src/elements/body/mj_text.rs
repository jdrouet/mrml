use super::BodyElement;
use crate::elements::body::prelude::*;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::{Context, Header, Tag};
use roxmltree::Node;
use std::collections::HashMap;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::new()
        .add("align", "left")
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("line-height", "1")
        .add("padding", "10px 25px");
}

#[derive(Clone, Debug)]
pub struct MJText {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJText {
    fn default_attributes<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<MJText, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        Ok(MJText {
            attributes: MJText::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }

    fn set_style_text(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("font-style", self.get_attribute("font-style"))
            .maybe_set_style("font-weight", self.get_attribute("font-weight"))
            .maybe_set_style("letter-spacing", self.get_attribute("letter-spacing"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("text-align", self.get_attribute("align"))
            .maybe_set_style("text-decoration", self.get_attribute("text-decoration"))
            .maybe_set_style("text-transform", self.get_attribute("text-transform"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("height", self.get_attribute("height"))
    }

    fn render_content(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        Ok(self.set_style_text(Tag::div()).render(res.join("")))
    }

    fn render_with_height(&self, header: &Header, height: &String) -> Result<String, Error> {
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .set_attribute("height", height)
            .set_style("height", height)
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
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}

impl BodyComponent for MJText {
    fn set_style(&self, name: &str, tag: Tag) -> Tag {
        match name {
            "text" => self.set_style_text(tag),
            _ => tag,
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
