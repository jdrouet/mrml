use crate::comment::Comment;
use crate::from_child;
use crate::mj_social_element::{MJSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::prelude::parse::{Error as ParserError, Parsable};
use crate::prelude::print::Print;
use crate::prelude::render::{Header, Render, Renderable};
use std::cell::RefCell;
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJSocialChild {
    Comment(Comment),
    MJSocialElement(MJSocialElement),
}

from_child!(MJSocialChild, Comment);
from_child!(MJSocialChild, MJSocialElement);

impl Print for MJSocialChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJSocialElement(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

impl Parsable for MJSocialChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_SOCIAL_ELEMENT => Ok(MJSocialElement::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJSocialChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MJSocialElement(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
