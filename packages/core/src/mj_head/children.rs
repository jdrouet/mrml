use crate::comment::Comment;
use crate::from_child;
use crate::mj_attributes::MJAttributes;
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::MJBreakpoint;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::MJFont;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_preview::MJPreview;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_title::MJTitle;
use crate::mj_title::NAME as MJ_TITLE;
use crate::prelude::parse::Error as ParserError;
use crate::prelude::print::Print;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJHeadChild {
    Comment(Comment),
    MJAttributes(MJAttributes),
    MJBreakpoint(MJBreakpoint),
    MJFont(MJFont),
    MJPreview(MJPreview),
    MJTitle(MJTitle),
}

from_child!(MJHeadChild, Comment);
from_child!(MJHeadChild, MJAttributes);
from_child!(MJHeadChild, MJBreakpoint);
from_child!(MJHeadChild, MJFont);
from_child!(MJHeadChild, MJPreview);
from_child!(MJHeadChild, MJTitle);

impl MJHeadChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJAttributes(elt) => elt,
            Self::MJBreakpoint(elt) => elt,
            Self::MJFont(elt) => elt,
            Self::MJPreview(elt) => elt,
            Self::MJTitle(elt) => elt,
        }
    }
}

impl MJHeadChild {
    pub fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ATTRIBUTES => Ok(MJAttributes::parse(tokenizer)?.into()),
            MJ_BREAKPOINT => Ok(MJBreakpoint::parse(tokenizer)?.into()),
            MJ_FONT => Ok(MJFont::parse(tokenizer)?.into()),
            MJ_PREVIEW => Ok(MJPreview::parse(tokenizer)?.into()),
            MJ_TITLE => Ok(MJTitle::parse(tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}
