use crate::comment::Comment;
use crate::from_child;
use crate::prelude::parse::{Error as ParserError, Parsable};
use crate::prelude::print::Print;
use crate::prelude::render::{Header, Render, Renderable};
use crate::text::Text;
use std::cell::RefCell;
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJSocialElementChild {
    Comment(Comment),
    Text(Text),
}

from_child!(MJSocialElementChild, Comment);
from_child!(MJSocialElementChild, Text);

impl Print for MJSocialElementChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::Text(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

impl Parsable for MJSocialElementChild {
    fn parse<'a>(tag: StrSpan<'a>, _tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        Err(ParserError::UnexpectedElement(tag.start()))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJSocialElementChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::Text(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
