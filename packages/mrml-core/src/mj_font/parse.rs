use super::{MJFont, MJFontAttributes};
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJFontParser(MJFontAttributes);

impl Parser for MJFontParser {
    type Output = MJFont;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJFont { attributes: self.0 })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        match name.as_str() {
            "name" => {
                self.0.name = value.to_string();
                Ok(())
            }
            "href" => {
                self.0.href = value.to_string();
                Ok(())
            }
            _ => Err(Error::UnexpectedAttribute(name.start())),
        }
    }
}

impl Parsable for MJFont {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJFontParser::default().parse(tokenizer)?.build()
    }
}
