use crate::comment::Comment;
use crate::from_child;
use crate::mj_social_element::{MJSocialElement, NAME as MJ_SOCIAL_ELEMENT};
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
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJSocialChild {
    Comment(Comment),
    MJSocialElement(MJSocialElement),
}

from_child!(MJSocialChild, Comment);
from_child!(MJSocialChild, MJSocialElement);

#[cfg(feature = "print")]
impl Print for MJSocialChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJSocialElement(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

#[cfg(feature = "parse")]
impl Parsable for MJSocialChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_SOCIAL_ELEMENT => Ok(MJSocialElement::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJSocialChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MJSocialElement(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
