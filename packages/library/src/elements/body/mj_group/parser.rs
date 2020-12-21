use super::MJGroup;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("direction", "ltr");
}

impl MJGroup {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJGroup, Error> {
        let mut children = vec![];
        let mut attrs = Attributes::default();
        attrs.set("mobile-width", "mobile-width");
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, Some(&attrs))?);
        }
        Ok(MJGroup {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }
}
