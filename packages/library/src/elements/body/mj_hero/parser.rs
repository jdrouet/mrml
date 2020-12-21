use super::MJHero;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("background-color", "#ffffff")
        .add("background-position", "center center")
        .add("height", "0px")
        .add("mode", "fixed-height")
        .add("padding", "0px")
        .add("vertical-align", "top");
}

impl MJHero {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJHero, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        Ok(MJHero {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }
}
