use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::Header;
use crate::Options;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJTitle {
    content: String,
}

impl MJTitle {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, _opts: &Options) -> Result<Self, Error> {
        Ok(Self {
            content: match node.text() {
                Some(value) => value.to_string(),
                None => String::from(""),
            },
        })
    }
}

impl HeadComponent for MJTitle {
    fn update_header(&self, header: &mut Header) {
        header.set_title(self.content.clone());
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
