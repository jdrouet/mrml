use super::MJPreview;
use crate::prelude::parse::{Error, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJPreviewParser(MJPreview);

impl Parser for MJPreviewParser {
    type Output = MJPreview;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.0 .0 = value.to_string();
        Ok(())
    }
}

impl MJPreview {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJPreviewParser::default().parse(tokenizer)?.build()
    }
}
