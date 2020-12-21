use super::MJImage;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border", "0")
        .add("height", "auto")
        .add("padding", "10px 25px")
        .add("target", "_blank")
        .add("font-size", "13px");
}

impl MJImage {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJImage, Error> {
        Ok(MJImage {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
        })
    }
}
