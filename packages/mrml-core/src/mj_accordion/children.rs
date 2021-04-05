use crate::comment::Comment;
use crate::mj_accordion_element::{MJAccordionElement, NAME as MJ_ACCORDION_ELEMENT};
use crate::prelude::parse::{Error as ParserError, Parsable};
use crate::prelude::print::Print;
use crate::prelude::render::{Header, Render, Renderable};
use crate::{as_child, from_child};
use std::cell::RefCell;
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJAccordionChild {
    Comment(Comment),
    MJAccordionElement(MJAccordionElement),
}

as_child!(MJAccordionChild, MJAccordionElement, as_element);
from_child!(MJAccordionChild, Comment);
from_child!(MJAccordionChild, MJAccordionElement);

impl Print for MJAccordionChild {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        match self {
            Self::Comment(elt) => elt.print(f, pretty, level, indent_size),
            Self::MJAccordionElement(elt) => elt.print(f, pretty, level, indent_size),
        }
    }
}

impl Parsable for MJAccordionChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ACCORDION_ELEMENT => Ok(MJAccordionElement::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJAccordionChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MJAccordionElement(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
