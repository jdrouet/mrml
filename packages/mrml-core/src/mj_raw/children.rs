use crate::comment::Comment;
use crate::from_child;
use crate::node::Node;
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "print")]
use crate::prelude::print::Print;
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
use crate::text::Text;
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(feature = "parse")]
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJRawChild {
    Comment(Comment),
    Node(Node<MJRawChild>),
    Text(Text),
}

from_child!(MJRawChild, Comment);
from_child!(MJRawChild, Text);

impl From<Node<MJRawChild>> for MJRawChild {
    fn from(value: Node<MJRawChild>) -> Self {
        Self::Node(value)
    }
}

#[cfg(feature = "print")]
impl MJRawChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

#[cfg(feature = "print")]
impl Print for MJRawChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}

#[cfg(feature = "parse")]
impl Parsable for MJRawChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        Ok(Node::<MJRawChild>::parse(tag, tokenizer)?.into())
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJRawChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(header),
            Self::Node(elt) => elt.renderer(header),
            Self::Text(elt) => elt.renderer(header),
        }
    }
}
