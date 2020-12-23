use super::{MJSection, DEFAULT_BACKGROUND_POSITION};
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("background-position", DEFAULT_BACKGROUND_POSITION)
        .add("background-repeat", "repeat")
        .add("background-size", "auto")
        .add("direction", "ltr")
        .add("padding", "20px 0")
        .add("text-align", "center")
        .add("text-padding", "4px 4px 4px 0");
}

impl MJSection {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSection, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        Ok(MJSection {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }
}
