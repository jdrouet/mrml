use super::MJFont;
use crate::elements::error::Error;
use crate::parser::MJMLParser;
use xmlparser::{StrSpan, Tokenizer};

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

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        match name.as_str() {
            "name" => self.name = Some(value.to_string()),
            "href" => self.href = Some(value.to_string()),
            _ => return Err(Error::UnexpectedAttribute(name.to_string())),
        };
        Ok(())
    }
}

impl MJFont {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJFontParser::default().parse(tokenizer)?.build()
    }
}
