use crate::comment::Comment;
use crate::from_child;
use crate::node::Node;
use crate::prelude::parse::{Error as ParserError, Parsable};
use crate::prelude::print::Print;
use crate::prelude::render::{Header, Render, Renderable};
use crate::text::Text;
use std::cell::RefCell;
use std::rc::Rc;
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

impl MJRawChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

impl Print for MJRawChild {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        self.as_print().print(f, pretty, level, indent_size)
    }
}

impl Parsable for MJRawChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        Ok(Node::<MJRawChild>::parse(tag, tokenizer)?.into())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJRawChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(header),
            Self::Node(elt) => elt.renderer(header),
            Self::Text(elt) => elt.renderer(header),
        }
    }
}
