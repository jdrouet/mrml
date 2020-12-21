use super::MJAccordion;
use crate::elements::body::mj_accordion_element::MJAccordionElement;
use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

const CHILDREN_ATTRIBUTES: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("border", "2px solid black")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("icon-align", "middle")
        .add("icon-position", "right")
        .add("icon-height", "32px")
        .add("icon-width", "32px")
        .add("icon-wrapped-url", "https://i.imgur.com/bIXv1bk.png")
        .add("icon-wrapped-alt", "+")
        .add("icon-unwrapped-url", "https://i.imgur.com/w4uTygT.png")
        .add("icon-unwrapped-alt", "-")
        .add("padding", "10px 25px");
}

impl MJAccordion {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJAccordion, Error> {
        let mut result = MJAccordion {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children: vec![],
        };
        let child_attrs = result.get_children_attributes();
        for child in node.children.iter() {
            match child {
                Element::Node(node) => match node.name.as_str() {
                    "mj-accordion-element" => {
                        let element = MJAccordionElement::parse(node, header, &child_attrs)?;
                        result
                            .children
                            .push(BodyElement::MJAccordionElement(element));
                    }
                    name => return Err(Error::UnexpectedElement(name.into())),
                },
                // TODO handle comments
                Element::Comment(_) => (),
                Element::Text(_) => return Err(Error::UnexpectedText),
            };
        }
        Ok(result)
    }

    fn get_children_attributes(&self) -> Attributes {
        let mut res = Attributes::default();
        for key in CHILDREN_ATTRIBUTES.iter() {
            if let Some(value) = self.get_attribute(key) {
                res.set(key, value);
            }
        }
        res
    }
}
