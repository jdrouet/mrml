use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::Header;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJPreview {
    content: Option<String>,
}

impl MJPreview {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>) -> Result<Self, Error> {
        let content = node.text().and_then(|value| Some(value.to_string()));
        Ok(Self { content })
    }

    pub fn get_content(&self) -> String {
        match self.content.as_ref() {
            Some(value) => value.to_string(),
            None => "".into(),
        }
    }
}

impl HeadComponent for MJPreview {
    fn update_header(&self, header: &mut Header) {
        header.set_preview(self.get_content());
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
