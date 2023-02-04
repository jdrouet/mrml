use super::{MjRaw, MjRawChild};
use crate::node::Node;
use crate::prelude::parse::{Error, Parsable, Parser, ParserOptions};
use crate::{parse_child, parse_comment, parse_text};
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjRawChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        Ok(Node::<MjRawChild>::parse(tag, tokenizer, opts)?.into())
    }
}

#[derive(Debug, Default)]
struct MjRawParser {
    opts: Rc<ParserOptions>,
    children: Vec<MjRawChild>,
}

impl MjRawParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            children: Vec::new(),
        }
    }
}

impl Parser for MjRawParser {
    type Output = MjRaw;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MjRaw {
            children: self.children,
        })
    }

    parse_child!(MjRawChild);
    parse_comment!();
    parse_text!();
}

impl Parsable for MjRaw {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjRawParser::new(opts).parse(tokenizer)?.build()
    }
}
