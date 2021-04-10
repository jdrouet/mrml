use crate::comment::Comment;
use crate::mj_accordion_text::{MJAccordionText, NAME as MJ_ACCORDION_TEXT};
use crate::mj_accordion_title::{MJAccordionTitle, NAME as MJ_ACCORDION_TITLE};
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "print")]
use crate::prelude::print::Print;
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
use crate::{as_child, from_child};
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(feature = "parse")]
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJAccordionElementChild {
    Comment(Comment),
    MJAccordionText(MJAccordionText),
    MJAccordionTitle(MJAccordionTitle),
}

as_child!(MJAccordionElementChild, MJAccordionText, as_text);
as_child!(MJAccordionElementChild, MJAccordionTitle, as_title);
from_child!(MJAccordionElementChild, Comment);
from_child!(MJAccordionElementChild, MJAccordionText);
from_child!(MJAccordionElementChild, MJAccordionTitle);

#[cfg(feature = "print")]
impl Print for MJAccordionElementChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJAccordionText(elt) => elt.print(pretty, level, indent_size),
            Self::MJAccordionTitle(elt) => elt.print(pretty, level, indent_size),
        }
    }
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
