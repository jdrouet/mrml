use crate::comment::Comment;
use crate::mj_accordion_text::MJAccordionText;
#[cfg(feature = "parse")]
use crate::mj_accordion_text::NAME as MJ_ACCORDION_TEXT;
use crate::mj_accordion_title::MJAccordionTitle;
#[cfg(feature = "parse")]
use crate::mj_accordion_title::NAME as MJ_ACCORDION_TITLE;
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
#[cfg(feature = "render")]
use std::cell::RefCell;
#[cfg(feature = "render")]
use std::rc::Rc;
#[cfg(feature = "parse")]
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MJAccordionElementChild {
    Comment(Comment),
    MJAccordionText(MJAccordionText),
    MJAccordionTitle(MJAccordionTitle),
}

#[cfg(feature = "parse")]
impl Parsable for MJAccordionElementChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ACCORDION_TEXT => Ok(MJAccordionText::parse(tag, tokenizer)?.into()),
            MJ_ACCORDION_TITLE => Ok(MJAccordionTitle::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJAccordionElementChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(header),
            Self::MJAccordionText(elt) => elt.renderer(header),
            Self::MJAccordionTitle(elt) => elt.renderer(header),
        }
    }
}
