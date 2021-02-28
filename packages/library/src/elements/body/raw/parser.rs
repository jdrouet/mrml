use super::RawElement;
use crate::elements::body::node::Node;
use crate::parser::Error;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

impl RawElement {
    pub fn conditional_parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<RawElement, Error> {
        Ok(Self::from(Node::conditional_parse(
            tag, tokenizer, header, only_raw,
        )?))
    }
}
