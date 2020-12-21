use super::MJText;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "left")
        .add("color", "#000000")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("line-height", "1")
        .add("padding", "10px 25px");
}

impl MJText {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJText, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        Ok(MJText {
            attributes: MJText::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }
}
