use super::MJButton;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("background-color", "#414141")
        .add("border", "none")
        .add("border-radius", "3px")
        .add("color", "#ffffff")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("font-weight", "normal")
        .add("inner-padding", "10px 25px")
        .add("line-height", "120%")
        .add("padding", "10px 25px")
        .add("target", "_blank")
        .add("text-decoration", "none")
        .add("text-transform", "none")
        .add("vertical-align", "middle");
}

impl MJButton {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJButton, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(&child, header, None::<&Attributes>)?);
        }
        Ok(MJButton {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children,
        })
    }
}
