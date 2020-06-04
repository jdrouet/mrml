use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::Header;
use crate::Options;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJPreview {
    content: String,
}

impl MJPreview {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, _opts: &Options) -> Result<Self, Error> {
        let content = match node.text() {
            Some(value) => value.to_string(),
            None => "".into(),
        };
        Ok(Self { content })
    }

    pub fn get_content(&self) -> String {
        self.content.clone()
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
