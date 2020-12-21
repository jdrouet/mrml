use super::MJSpacer;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("height", "20px");
}

impl MJSpacer {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSpacer, Error> {
        Ok(MJSpacer {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
        })
    }
}
