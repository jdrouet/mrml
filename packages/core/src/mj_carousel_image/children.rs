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
pub enum MJCarouselImageChild {
    Comment(Comment),
    Text(Text),
}

from_child!(MJCarouselImageChild, Comment);
from_child!(MJCarouselImageChild, Text);

impl Print for MJCarouselImageChild {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        match self {
            Self::Comment(elt) => elt.print(f, pretty, level, indent_size),
            Self::Text(elt) => elt.print(f, pretty, level, indent_size),
        }
    }
}

impl Parsable for MJCarouselImageChild {
    fn parse<'a>(tag: StrSpan<'a>, _tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        Err(ParserError::UnexpectedElement(tag.start()))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJCarouselImageChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::Text(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
