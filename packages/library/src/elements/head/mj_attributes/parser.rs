use super::MJAttributes;
use crate::elements::error::Error;
use crate::elements::head::{mj_attributes_all, mj_attributes_class, mj_attributes_element};
use crate::parser::{Element, Node};

impl MJAttributes {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        let mut element = Self::new();
        for child in node.children.iter() {
            element.parse_child(&child)?;
        }
        Ok(element)
    }

    fn parse_child<'a>(&mut self, element: &Element<'a>) -> Result<(), Error> {
        match element {
            Element::Node(node) => match node.name.as_str() {
                mj_attributes_all::NAME => self.parse_all(node),
                mj_attributes_class::NAME => self.parse_class(node),
                _ => self.parse_element(node),
            },
            // TODO handle comments
            Element::Comment(_) => Ok(()),
            Element::Text(_) => Err(Error::UnexpectedText),
        }
    }

    fn parse_all<'a>(&mut self, node: &Node<'a>) -> Result<(), Error> {
        self.children
            .push(mj_attributes_all::MJAttributesAll::parse(node)?.into());
        Ok(())
    }

    fn parse_class<'a>(&mut self, node: &Node<'a>) -> Result<(), Error> {
        self.children
            .push(mj_attributes_class::MJAttributesClass::parse(node)?.into());
        Ok(())
    }

    fn parse_element<'a>(&mut self, node: &Node<'a>) -> Result<(), Error> {
        self.children
            .push(mj_attributes_element::MJAttributesElement::parse(node)?.into());
        Ok(())
    }
}
