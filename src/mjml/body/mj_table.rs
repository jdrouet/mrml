use crate::mjml::body::prelude::*;
use crate::mjml::body::raw::RawElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::attributes::*;
use crate::util::{Context, Header, Tag};
use roxmltree::Node;
use std::collections::HashMap;

fn create_default_attributes() -> Attributes {
    Attributes::new()
        .add("align", "left")
        .add("border", "none")
        .add("cellpadding", "0")
        .add("cellspacing", "0")
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("line-height", "22px")
        .add("padding", "10px 25px")
        .add("table-layout", "auto")
        .add("width", "100%")
}

#[derive(Clone, Debug)]
pub struct MJTable {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJTable {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Result<MJTable, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(RawElement::conditional_parse(&child, header, true)?);
        }
        Ok(MJTable {
            attributes: create_default_attributes().concat(node),
            context: None,
            children,
        })
    }

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.maybe_set_style("border", self.get_attribute("border"))
            .maybe_set_style("color", self.get_attribute("color"))
            .maybe_set_style("font-family", self.get_attribute("font-family"))
            .maybe_set_style("font-size", self.get_attribute("font-size"))
            .maybe_set_style("line-height", self.get_attribute("line-height"))
            .maybe_set_style("table-layout", self.get_attribute("table-layout"))
            .maybe_set_style("width", self.get_attribute("width"))
    }
}

impl Component for MJTable {
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
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        let table = self
            .set_style_table(Tag::new("table"))
            .set_attribute("border", 0)
            .maybe_set_attribute("cellpadding", self.get_attribute("cellpadding"))
            .maybe_set_attribute("cellspacing", self.get_attribute("cellspacing"))
            .maybe_set_attribute("width", self.get_attribute("width"));
        Ok(table.render(res.join("")))
    }
}

impl BodyComponent for MJTable {}
impl BodyContainedComponent for MJTable {}
impl ComponentWithAttributes for MJTable {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(self.attributes.inner())
    }
}
impl ComponentWithSizeAttribute for MJTable {}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-table.mjml"),
            include_str!("../../../test/mj-table.html"),
        );
    }

    #[test]
    fn with_text_attributes() {
        compare_render(
            include_str!("../../../test/mj-table-text.mjml"),
            include_str!("../../../test/mj-table-text.html"),
        );
    }

    #[test]
    fn with_table_attributes() {
        compare_render(
            include_str!("../../../test/mj-table-table.mjml"),
            include_str!("../../../test/mj-table-table.html"),
        );
    }

    #[test]
    fn with_other_attributes() {
        compare_render(
            include_str!("../../../test/mj-table-other.mjml"),
            include_str!("../../../test/mj-table-other.html"),
        );
    }
}
