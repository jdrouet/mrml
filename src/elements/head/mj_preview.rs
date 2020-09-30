use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJPreview {
    pub content: String,
}

impl MJPreview {
    pub fn parse<'a>(node: &Node<'a>) -> Result<Self, Error> {
        for child in node.children.iter() {
            match child {
                Element::Text(value) => {
                    return Ok(Self {
                        content: value.as_str().into(),
                    });
                }
                _ => return Err(Error::InvalidChild),
            };
        }
        Ok(Self {
            content: String::new(),
        })
    }
}

impl HeadComponent for MJPreview {
    fn update_header(&self, header: &mut Header) {
        header.set_preview(self.content.clone());
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::{compare_preview, compare_render};

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-preview.mjml"),
            include_str!("../../../test/mj-preview.html"),
        );
    }

    #[test]
    fn to_preview() {
        compare_preview(include_str!("../../../test/mj-preview.mjml"), "Hello MJML");
    }
}
