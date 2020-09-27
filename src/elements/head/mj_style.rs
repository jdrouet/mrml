use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::Header;

#[derive(Clone, Debug)]
pub struct MJStyle {
    content: String,
    inline: bool,
}

impl MJStyle {
    pub fn parse<'a>(node: &Node<'a>) -> Result<Self, Error> {
        let mut content = String::new();
        let mut inline = false;
        for (key, _value) in node.attributes.iter() {
            match key.as_str() {
                "inline" => {
                    inline = true;
                }
                _ => return Err(Error::ParseError("unexpected attribute".into())),
            };
        }
        for child in node.children.iter() {
            match child {
                Element::Text(value) => content.push_str(value.as_str()),
                _ => return Err(Error::ParseError("unexpected child".into())),
            };
        }
        Ok(Self { inline, content })
    }
}

impl HeadComponent for MJStyle {
    fn update_header(&self, header: &mut Header) {
        if self.inline {
            // TODO
        } else {
            header.add_style(self.content.clone());
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-style.mjml"),
            include_str!("../../../test/mj-style.html"),
        );
    }
}
