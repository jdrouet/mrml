use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::header::{DefaultAttributes, Header};

#[derive(Clone, Debug)]
pub struct MJAttributes(DefaultAttributes);

impl MJAttributes {
    fn new() -> Self {
        Self(DefaultAttributes::default())
    }

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
                "mj-all" => self.parse_all(node),
                "mj-class" => self.parse_class(node),
                _ => self.parse_element(node),
            },
            // TODO handle comments
            Element::Comment(_) => (),
            Element::Text(_) => return Err(Error::UnexpectedText),
        };
        Ok(())
    }

    fn parse_all<'a>(&mut self, node: &Node<'a>) {
        self.0.add_all_content(
            node.attributes
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str())),
        );
    }

    fn parse_class<'a>(&mut self, node: &Node<'a>) {
        let name = node
            .attributes
            .iter()
            .find(|(key, _value)| key.as_str() == "name")
            .map(|(_key, value)| value.as_str());
        if let Some(name) = name {
            self.0.add_class_content(
                name,
                node.attributes.iter().filter_map(|(key, value)| {
                    let key = key.as_str();
                    if key == "name" {
                        None
                    } else {
                        Some((key, value.as_str()))
                    }
                }),
            )
        }
    }

    fn parse_element<'a>(&mut self, node: &Node<'a>) {
        self.0.add_element_content(
            node.name.as_str(),
            node.attributes
                .iter()
                .map(|(key, value)| (key.as_str(), value.as_str())),
        );
    }
}

impl HeadComponent for MJAttributes {
    fn update_header(&self, header: &mut Header) {
        header.set_default_attributes(self.0.clone());
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-attributes.mjml"),
            include_str!("../../../test/mj-attributes.html"),
        );
    }
}
