use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::Header;
use crate::Options;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJStyle {
    content: String,
    inline: bool,
}

impl MJStyle {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, _opts: &Options) -> Result<Self, Error> {
        let inline = match node.attribute("inline") {
            Some(value) => value == "inline",
            None => false,
        };
        let content = match node.text() {
            Some(value) => value.to_string(),
            None => "".into(),
        };
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
