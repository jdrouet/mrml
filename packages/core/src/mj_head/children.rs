use crate::comment::Comment;
use crate::from_child;
use crate::mj_title::MJTitle;
use crate::mj_title::NAME as MJ_TITLE;
use crate::prelude::parse::Error as ParserError;
use crate::prelude::print::Print;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJHeadChild {
    Comment(Comment),
    MJTitle(MJTitle),
}

from_child!(MJHeadChild, Comment);
from_child!(MJHeadChild, MJTitle);

impl MJHeadChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJTitle(elt) => elt,
        }
    }
}

impl MJHeadChild {
    pub fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_TITLE => Ok(MJTitle::parse(tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}
