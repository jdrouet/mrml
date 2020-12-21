use super::MJAccordionElement;
use crate::elements::body::mj_accordion_text::MJAccordionText;
use crate::elements::body::mj_accordion_title::MJAccordionTitle;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

impl MJAccordionElement {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, Attributes::default())
    }

    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        attrs: &Attributes,
    ) -> Result<MJAccordionElement, Error> {
        if node.name.as_str() != "mj-accordion-element" {
            return Err(Error::UnexpectedElement(node.name.as_str().into()));
        }
        let mut element = MJAccordionElement {
            attributes: Self::default_attributes(node, header)
                .concat(attrs)
                .concat(node),
            context: None,
            title: None,
            text: None,
        };
        let children_attr = element.get_children_attributes();
        for child in node.children.iter() {
            match child {
                Element::Node(node) => match node.name.as_str() {
                    "mj-accordion-title" => {
                        element.title =
                            Some(MJAccordionTitle::parse(node, header, &children_attr)?);
                    }
                    "mj-accordion-text" => {
                        element.text = Some(MJAccordionText::parse(node, header, &children_attr)?);
                    }
                    name => return Err(Error::UnexpectedElement(name.into())),
                },
                // TODO handle comments
                Element::Comment(_) => (),
                Element::Text(_) => return Err(Error::UnexpectedText),
            };
        }
        Ok(element)
    }
}
