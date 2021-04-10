use crate::comment::Comment;
use crate::from_child;
use crate::mj_navbar_link::{MJNavbarLink, NAME as MJ_NAVBAR_LINK};
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "print")]
use crate::prelude::print::Print;
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(feature = "parse")]
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJNavbarChild {
    Comment(Comment),
    MJNavbarLink(MJNavbarLink),
}

from_child!(MJNavbarChild, Comment);
from_child!(MJNavbarChild, MJNavbarLink);

#[cfg(feature = "print")]
impl Print for MJNavbarChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJNavbarLink(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

#[cfg(feature = "parse")]
impl Parsable for MJNavbarChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_NAVBAR_LINK => Ok(MJNavbarLink::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJNavbarChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MJNavbarLink(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
