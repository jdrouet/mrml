use crate::comment::Comment;
use crate::from_child;
use crate::mj_button::MJButton;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_column::MJColumn;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::MJDivider;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_group::MJGroup;
use crate::mj_group::NAME as MJ_GROUP;
use crate::mj_hero::MJHero;
use crate::mj_hero::NAME as MJ_HERO;
use crate::mj_image::MJImage;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_raw::MJRaw;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::MJSection;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_spacer::MJSpacer;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_text::MJText;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::MJWrapper;
use crate::mj_wrapper::NAME as MJ_WRAPPER;
use crate::node::Node;
use crate::prelude::parse::{Error as ParserError, Parsable};
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
    MJGroup(MJGroup),
    MJHero(MJHero),
    MJImage(MJImage),
    MJRaw(MJRaw),
    MJSection(MJSection),
    MJSpacer(MJSpacer),
    MJText(MJText),
    MJWrapper(MJWrapper),
    Node(Node<MJBodyChild>),
    Text(Text),
}

from_child!(MJBodyChild, Comment);
from_child!(MJBodyChild, MJButton);
from_child!(MJBodyChild, MJColumn);
from_child!(MJBodyChild, MJDivider);
from_child!(MJBodyChild, MJGroup);
from_child!(MJBodyChild, MJHero);
from_child!(MJBodyChild, MJImage);
from_child!(MJBodyChild, MJRaw);
from_child!(MJBodyChild, MJSection);
from_child!(MJBodyChild, MJSpacer);
from_child!(MJBodyChild, MJText);
from_child!(MJBodyChild, MJWrapper);
from_child!(MJBodyChild, Text);

impl From<Node<MJBodyChild>> for MJBodyChild {
    fn from(value: Node<MJBodyChild>) -> Self {
        Self::Node(value)
    }
}

impl MJBodyChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJButton(elt) => elt,
            Self::MJColumn(elt) => elt,
            Self::MJDivider(elt) => elt,
            Self::MJGroup(elt) => elt,
            Self::MJHero(elt) => elt,
            Self::MJImage(elt) => elt,
            Self::MJRaw(elt) => elt,
            Self::MJSection(elt) => elt,
            Self::MJSpacer(elt) => elt,
            Self::MJText(elt) => elt,
            Self::MJWrapper(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

impl Print for MJBodyChild {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        self.as_print().print(f, pretty, level, indent_size)
    }
}

impl Parsable for MJBodyChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_BUTTON => Ok(MJButton::parse(tag, tokenizer)?.into()),
            MJ_COLUMN => Ok(MJColumn::parse(tag, tokenizer)?.into()),
            MJ_DIVIDER => Ok(MJDivider::parse(tag, tokenizer)?.into()),
            MJ_GROUP => Ok(MJGroup::parse(tag, tokenizer)?.into()),
            MJ_HERO => Ok(MJHero::parse(tag, tokenizer)?.into()),
            MJ_IMAGE => Ok(MJImage::parse(tag, tokenizer)?.into()),
            MJ_RAW => Ok(MJRaw::parse(tag, tokenizer)?.into()),
            MJ_SECTION => Ok(MJSection::parse(tag, tokenizer)?.into()),
            MJ_SPACER => Ok(MJSpacer::parse(tag, tokenizer)?.into()),
            MJ_TEXT => Ok(MJText::parse(tag, tokenizer)?.into()),
            MJ_WRAPPER => Ok(MJWrapper::parse(tag, tokenizer)?.into()),
            _ => Ok(Node::<MJBodyChild>::parse(tag, tokenizer)?.into()),
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
            Self::MJGroup(elt) => elt.renderer(header),
            Self::MJHero(elt) => elt.renderer(header),
            Self::MJImage(elt) => elt.renderer(header),
            Self::MJRaw(elt) => elt.renderer(header),
            Self::MJSection(elt) => elt.renderer(header),
            Self::MJSpacer(elt) => elt.renderer(header),
            Self::MJText(elt) => elt.renderer(header),
            Self::MJWrapper(elt) => elt.renderer(header),
            Self::Node(elt) => elt.renderer(header),
            Self::Text(elt) => elt.renderer(header),
        }
    }
}
