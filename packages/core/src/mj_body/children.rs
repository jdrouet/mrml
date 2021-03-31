use crate::comment::Comment;
use crate::from_child;
use crate::mj_button::MJButton;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_column::MJColumn;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::MJDivider;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_image::MJImage;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_section::MJSection;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_text::MJText;
use crate::mj_text::NAME as MJ_TEXT;
use crate::node::Node;
use crate::prelude::parse::Error as ParserError;
use crate::prelude::print::Print;
use crate::prelude::render::{Header, Render, Renderable};
use crate::text::Text;
use std::cell::RefCell;
use std::rc::Rc;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJBodyChild {
    Comment(Comment),
    MJButton(MJButton),
    MJColumn(MJColumn),
    MJDivider(MJDivider),
    MJImage(MJImage),
    MJSection(MJSection),
    MJText(MJText),
    Node(Node),
    Text(Text),
}

from_child!(MJBodyChild, Comment);
from_child!(MJBodyChild, MJButton);
from_child!(MJBodyChild, MJColumn);
from_child!(MJBodyChild, MJDivider);
from_child!(MJBodyChild, MJImage);
from_child!(MJBodyChild, MJSection);
from_child!(MJBodyChild, MJText);
from_child!(MJBodyChild, Node);
from_child!(MJBodyChild, Text);

impl MJBodyChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJButton(elt) => elt,
            Self::MJColumn(elt) => elt,
            Self::MJDivider(elt) => elt,
            Self::MJImage(elt) => elt,
            Self::MJSection(elt) => elt,
            Self::MJText(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

impl MJBodyChild {
    pub fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_BUTTON => Ok(MJButton::parse(tokenizer)?.into()),
            MJ_COLUMN => Ok(MJColumn::parse(tokenizer)?.into()),
            MJ_DIVIDER => Ok(MJDivider::parse(tokenizer)?.into()),
            MJ_IMAGE => Ok(MJImage::parse(tokenizer)?.into()),
            MJ_SECTION => Ok(MJSection::parse(tokenizer)?.into()),
            MJ_TEXT => Ok(MJText::parse(tokenizer)?.into()),
            _ => Ok(Node::parse(tag.to_string(), tokenizer)?.into()),
            // _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJBodyChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(header),
            Self::MJButton(elt) => elt.renderer(header),
            Self::MJColumn(elt) => elt.renderer(header),
            Self::MJDivider(elt) => elt.renderer(header),
            Self::MJImage(elt) => elt.renderer(header),
            Self::MJSection(elt) => elt.renderer(header),
            Self::MJText(elt) => elt.renderer(header),
            Self::Node(elt) => elt.renderer(header),
            Self::Text(elt) => elt.renderer(header),
        }
    }
}
