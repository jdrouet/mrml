use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::header::DefaultAttributes;
use crate::util::Header;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJAttributes(DefaultAttributes);

impl MJAttributes {
    fn new() -> Self {
        Self(DefaultAttributes::new())
    }

    pub fn parse<'a, 'b>(node: &Node<'a, 'b>) -> Result<Self, Error> {
        let mut element = Self::new();
        for child in node.children() {
            element.parse_child(&child);
        }
        Ok(element)
    }

    fn parse_child<'a, 'b>(&mut self, node: &Node<'a, 'b>) {
        match node.tag_name().name() {
            "mj-all" => self.parse_all(node),
            "mj-class" => self.parse_class(node),
            "" => (),
            _ => self.parse_element(node),
        };
    }

    fn parse_all<'a, 'b>(&mut self, node: &Node<'a, 'b>) {
        self.0.add_all_content(
            node.attributes()
                .iter()
                .map(|attr| (attr.name(), attr.value())),
        );
    }

    fn parse_class<'a, 'b>(&mut self, node: &Node<'a, 'b>) {
        if let Some(classname) = node.attribute("name") {
            self.0.add_class_content(
                classname,
                node.attributes().iter().filter_map(|attr| {
                    let key = attr.name();
                    let value = attr.value();
                    if key == "name" {
                        None
                    } else {
                        Some((key, value))
                    }
                }),
            );
        }
    }

    fn parse_element<'a, 'b>(&mut self, node: &Node<'a, 'b>) {
        self.0.add_element_content(
            node.tag_name().name(),
            node.attributes()
                .iter()
                .map(|attr| (attr.name(), attr.value())),
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
