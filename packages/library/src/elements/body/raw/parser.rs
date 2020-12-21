use super::{NodeElement, RawElement};
use crate::elements::body::BodyElement;
use crate::elements::Error;
use crate::parser::{Element, Node};
use crate::util::attributes::Attributes;
use crate::util::header::Header;

impl NodeElement {
    fn conditional_parse<'a>(
        node: &Node<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<NodeElement, Error> {
        let tag = node.name.as_str();
        if only_raw && tag.starts_with("mj-") {
            return Err(Error::UnexpectedElement(tag.into()));
        }
        let mut children = vec![];
        for child in node.children.iter() {
            if only_raw {
                children.push(BodyElement::Raw(RawElement::conditional_parse(
                    child, header, true,
                )?))
            } else {
                children.push(BodyElement::parse(&child, header, None)?);
            }
        }
        Ok(NodeElement {
            attributes: Attributes::from(node),
            context: None,
            children,
            tag: node.name.as_str().to_string(),
        })
    }
}

impl RawElement {
    pub fn parse<'a>(element: &Element<'a>, header: &Header) -> Result<RawElement, Error> {
        RawElement::conditional_parse(element, header, false)
    }

    pub fn conditional_parse<'a>(
        element: &Element<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        match element {
            Element::Text(value) => Ok(RawElement::Text(value.as_str().into())),
            Element::Comment(value) => Ok(RawElement::Comment(value.as_str().into())),
            Element::Node(node) => Ok(RawElement::Node(NodeElement::conditional_parse(
                node, header, only_raw,
            )?)),
        }
    }
}
