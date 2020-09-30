use super::prelude::*;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::header::Header;

#[derive(Clone, Debug)]
pub struct MJFont {
    name: String,
    href: String,
}

impl MJFont {
    pub fn parse<'a>(node: &Node<'a>) -> Result<Self, Error> {
        let mut name: Option<String> = None;
        let mut href: Option<String> = None;
        for (key, value) in node.attributes.iter() {
            let key = key.as_str();
            match key {
                "name" => {
                    name = Some(value.as_str().into());
                }
                "href" => {
                    href = Some(value.as_str().into());
                }
                _ => return Err(Error::UnexpectedAttribute(key.into())),
            };
        }
        if name.is_none() {
            return Err(Error::MissingAttribute("name".into()));
        }
        if href.is_none() {
            return Err(Error::MissingAttribute("href".into()));
        }
        let name = name.unwrap();
        let href = href.unwrap();
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
