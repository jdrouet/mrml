use crate::comment::Comment;
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
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "print")]
use crate::prelude::print::Print;
use crate::{as_child, from_child};
#[cfg(feature = "parse")]
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
as_child!(MJHeadChild, MJAttributes, as_mj_attributes);
from_child!(MJHeadChild, MJAttributes);
as_child!(MJHeadChild, MJBreakpoint, as_mj_breakpoint);
from_child!(MJHeadChild, MJBreakpoint);
as_child!(MJHeadChild, MJFont, as_mj_font);
from_child!(MJHeadChild, MJFont);
as_child!(MJHeadChild, MJPreview, as_mj_preview);
from_child!(MJHeadChild, MJPreview);
as_child!(MJHeadChild, MJTitle, as_mj_title);
from_child!(MJHeadChild, MJTitle);

#[cfg(feature = "print")]
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

#[cfg(feature = "print")]
impl Print for MJHeadChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}

#[cfg(feature = "parse")]
impl Parsable for MJHeadChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ATTRIBUTES => Ok(MJAttributes::parse(tag, tokenizer)?.into()),
            MJ_BREAKPOINT => Ok(MJBreakpoint::parse(tag, tokenizer)?.into()),
            MJ_FONT => Ok(MJFont::parse(tag, tokenizer)?.into()),
            MJ_PREVIEW => Ok(MJPreview::parse(tag, tokenizer)?.into()),
            MJ_TITLE => Ok(MJTitle::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}
