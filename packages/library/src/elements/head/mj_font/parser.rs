use super::MJFont;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};

#[derive(Default)]
struct MJFontParser {
    name: Option<String>,
    href: Option<String>,
}

impl MJMLParser for MJFontParser {
    type Output = MJFont;

    fn build(self) -> Result<Self::Output, Error> {
        if let Some(name) = self.name {
            if let Some(href) = self.href {
                Ok(MJFont { name, href })
            } else {
                Err(Error::MissingAttribute("href".into()))
            }
        } else {
            Err(Error::MissingAttribute("name".into()))
        }
    }

    fn parse(mut self, node: &Node) -> Result<Self, Error> {
        for (key, value) in node.attributes.iter() {
            match key.as_str() {
                "name" => {
                    self.name = Some(value.to_string());
                }
                "href" => {
                    self.href = Some(value.to_string());
                }
                _ => return Err(Error::UnexpectedAttribute(key.to_string())),
            };
        }
        Ok(self)
    }
}

impl MJFont {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJFontParser::default().parse(node)?.build()
    }
}
