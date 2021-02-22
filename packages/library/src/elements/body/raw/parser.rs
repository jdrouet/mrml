use super::RawElement;
use crate::elements::body::node::Node;
use crate::elements::Error;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

impl RawElement {
    pub fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
    ) -> Result<RawElement, Error> {
        RawElement::conditional_parse(tag, tokenizer, header, false)
    }

    pub fn conditional_parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        Ok(RawElement::Node(Node::conditional_parse(
            tag, tokenizer, header, only_raw,
        )?))
    }
}
