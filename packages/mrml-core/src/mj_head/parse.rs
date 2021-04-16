use super::{MJHead, MJHeadChild};
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJHeadParser(MJHead);

impl Parser for MJHeadParser {
    type Output = MJHead;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.0.children.push(MJHeadChild::parse(tag, tokenizer)?);
        Ok(())
    }
}

impl Parsable for MJHead {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJHeadParser::default().parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn unexpected_element() {
        let res = crate::mjml::MJML::parse(
            r#"<mjml><mj-head><mj-text>Hello World!</mj-text></mj-head></mjml>"#,
        );
        assert!(res.is_err());
    }
}
