use crate::elements::body::prelude::*;
use crate::elements::body::raw::RawElement;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::new()
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
        .add("width", "100%");
}

#[derive(Clone, Debug)]
pub struct MJTable {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJTable {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJTable, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::Raw(RawElement::conditional_parse(
                &child, header, true,
            )?));
        }
        Ok(MJTable {
            attributes: Self::default_attributes(node, header).concat(node),
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

impl BodyComponent for MJTable {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }
}

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
