use super::MJTable;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
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

impl MJTable {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJTable, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(RawElement::conditional_parse(&child, header, true)?.into());
        }
        Ok(MJTable {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }
}
