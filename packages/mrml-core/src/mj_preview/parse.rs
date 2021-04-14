use super::MJPreview;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJPreviewParser(MJPreview);

impl Parser for MJPreviewParser {
    type Output = MJPreview;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.0.children = value.to_string();
        Ok(())
    }
}

impl Parsable for MJPreview {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJPreviewParser::default().parse(tokenizer)?.build()
    }
}
