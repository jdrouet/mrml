use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::Header;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJTitle {
    content: Option<String>,
}

impl MJTitle {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>) -> Result<Self, Error> {
        Ok(Self {
            content: node.text().and_then(|value| Some(value.to_string())),
        })
    }

    pub fn get_content(&self) -> String {
        match self.content.as_ref() {
            Some(value) => value.to_string(),
            None => "".into(),
        }
    }
}

impl HeadComponent for MJTitle {
    fn update_header(&self, header: &mut Header) {
        header.set_title(self.get_content());
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::{compare_render, compare_title};

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-title.mjml"),
            include_str!("../../../test/mj-title.html"),
        );
    }

    #[test]
    fn to_title() {
        compare_title(include_str!("../../../test/mj-title.mjml"), "Hello MJML");
    }
}
