use super::MJPreview;
use crate::parser::{Error, MJMLParser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Default)]
struct MJPreviewParser {
    content: String,
}

impl MJMLParser for MJPreviewParser {
    type Output = MJPreview;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJPreview {
            content: self.content,
        })
    }

    fn parse_child_text(&mut self, content: StrSpan) -> Result<(), Error> {
        self.content.push_str(content.as_str());
        Ok(())
    }
}

impl MJPreview {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJPreviewParser::default().parse(tokenizer)?.build()
    }
}
