use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::{Element, Node};
use crate::util::header::Header;

#[derive(Clone, Debug, Default)]
pub struct MJPreview {
    pub content: String,
}

impl MJPreview {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        match node.children.first() {
            Some(element) => match element {
                Element::Text(value) => Ok(Self {
                    content: value.as_str().into(),
                }),
                _ => Err(Error::InvalidChild),
            },
            None => Ok(Self::default()),
        }
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
