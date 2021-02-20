use super::{MJAttributes, MjAttributesChild};
use crate::elements::error::Error;
use crate::elements::head::{mj_attributes_all, mj_attributes_class, mj_attributes_element};
use crate::parser::{Element, MJMLParser, Node};

#[derive(Default)]
struct MJAttributesParser {
    children: Vec<MjAttributesChild>,
}

impl MJMLParser for MJAttributesParser {
    type Output = MJAttributes;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAttributes {
            children: self.children,
        })
    }

    fn parse(mut self, node: &Node) -> Result<Self, Error> {
        for element in node.children.iter() {
            match element {
                Element::Node(node) => match node.name.as_str() {
                    mj_attributes_all::NAME => self
                        .children
                        .push(mj_attributes_all::MJAttributesAll::parse(node)?.into()),
                    mj_attributes_class::NAME => self
                        .children
                        .push(mj_attributes_class::MJAttributesClass::parse(node)?.into()),
                    _ => self
                        .children
                        .push(mj_attributes_element::MJAttributesElement::parse(node)?.into()),
                },
                // TODO handle comments
                Element::Comment(_) => {}
                Element::Text(_) => return Err(Error::UnexpectedText),
            };
        }
        Ok(self)
    }
}

impl MJAttributes {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJAttributesParser::default().parse(node)?.build()
    }
}
