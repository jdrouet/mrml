use super::MJStyle;
use crate::parser::{Error, MJMLParser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Default)]
struct MJStyleParser {
    inline: bool,
    content: Vec<String>,
}

impl MJMLParser for MJStyleParser {
    type Output = MJStyle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJStyle {
            inline: self.inline,
            content: self.content.join(""),
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, _value: StrSpan<'a>) -> Result<(), Error> {
        if name.as_str() == "inline" {
            self.inline = true;
            Ok(())
        } else {
            Err(Error::UnexpectedAttribute(name.start()))
        }
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.content.push(value.to_string());
        Ok(())
    }
}

impl MJStyle {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJStyleParser::default().parse(tokenizer)?.build()
    }
}
