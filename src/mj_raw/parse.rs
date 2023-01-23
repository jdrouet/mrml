use super::{MjRaw, MjRawChild};
use crate::node::Node;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_child, parse_comment, parse_text};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjRawChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        Ok(Node::<MjRawChild>::parse(tag, tokenizer)?.into())
    }
}

#[derive(Debug, Default)]
struct MjRawParser(MjRaw);

impl Parser for MjRawParser {
    type Output = MjRaw;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_child!(MjRawChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MjRaw {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MjRawParser::default().parse(tokenizer)?.build()
    }
}
