use super::prelude::*;
use crate::mjml::error::Error;
use crate::util::Header;
use roxmltree::Node;

#[derive(Clone, Debug)]
pub struct MJFont {
    name: String,
    href: String,
}

impl MJFont {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>) -> Result<Self, Error> {
        let name = match node.attribute("name") {
            Some(value) => value.to_string(),
            None => return Err(Error::ParseError("name attribute missing".into())),
        };
        let href = match node.attribute("href") {
            Some(value) => value.to_string(),
            None => return Err(Error::ParseError("href attribute missing".into())),
        };
        Ok(Self { name, href })
    }
}

impl HeadComponent for MJFont {
    fn update_header(&self, header: &mut Header) {
        header.register_font(self.name.as_str(), self.href.as_str());
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-font.mjml"),
            include_str!("../../../test/mj-font.html"),
        );
    }
}
