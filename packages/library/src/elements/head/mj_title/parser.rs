use super::MJTitle;
use crate::parser::{Error, MJMLParser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Default)]
struct MJTitleParser {
    content: String,
}

impl MJMLParser for MJTitleParser {
    type Output = MJTitle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJTitle {
            content: self.content,
        })
    }

    fn parse_child_text(&mut self, content: StrSpan) -> Result<(), Error> {
        self.content.push_str(content.as_str());
        Ok(())
    }
}

impl MJTitle {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJTitleParser::default().parse(tokenizer)?.build()
    }
}
