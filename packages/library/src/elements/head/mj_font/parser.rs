use super::MJFont;
use crate::elements::error::Error;
use crate::parser::Node;

impl MJFont {
    pub fn parse(node: &Node) -> Result<Self, Error> {
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
