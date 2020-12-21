use super::MJDivider;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("border-color", "#000000")
        .add("border-style", "solid")
        .add("border-width", "4px")
        .add("padding", "10px 25px")
        .add("width", "100%");
}

impl MJDivider {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJDivider, Error> {
        Ok(MJDivider {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
        })
    }
}
