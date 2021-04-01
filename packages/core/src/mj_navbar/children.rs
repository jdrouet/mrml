use crate::comment::Comment;
use crate::from_child;
use crate::mj_navbar_link::{MJNavbarLink, NAME as MJ_NAVBAR_LINK};
use crate::prelude::parse::{Error as ParserError, Parsable};
use crate::prelude::print::Print;
use crate::prelude::render::{Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJNavbarChild {
    Comment(Comment),
    MJNavbarLink(MJNavbarLink),
}

from_child!(MJNavbarChild, Comment);
from_child!(MJNavbarChild, MJNavbarLink);

impl Print for MJNavbarChild {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        match self {
            Self::Comment(elt) => elt.print(f, pretty, level, indent_size),
            Self::MJNavbarLink(elt) => elt.print(f, pretty, level, indent_size),
        }
    }
}

impl Parsable for MJNavbarChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_NAVBAR_LINK => Ok(MJNavbarLink::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJNavbarChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MJNavbarLink(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
