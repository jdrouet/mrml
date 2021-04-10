use crate::comment::Comment;
use crate::from_child;
use crate::mj_accordion::MJAccordion;
use crate::mj_accordion::NAME as MJ_ACCORDION;
use crate::mj_button::MJButton;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_carousel::MJCarousel;
use crate::mj_carousel::NAME as MJ_CAROUSEL;
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
use crate::mj_navbar::MJNavbar;
use crate::mj_navbar::NAME as MJ_NAVBAR;
use crate::mj_raw::MJRaw;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::MJSection;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_social::MJSocial;
use crate::mj_social::NAME as MJ_SOCIAL;
use crate::mj_spacer::MJSpacer;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_text::MJText;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::MJWrapper;
use crate::mj_wrapper::NAME as MJ_WRAPPER;
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
pub enum MJBodyChild {
    Comment(Comment),
    MJAccordion(MJAccordion),
    MJButton(MJButton),
    MJCarousel(MJCarousel),
    MJColumn(MJColumn),
    MJDivider(MJDivider),
    MJGroup(MJGroup),
    MJHero(MJHero),
    MJImage(MJImage),
    MJNavbar(MJNavbar),
    MJRaw(MJRaw),
    MJSection(MJSection),
    MJSocial(MJSocial),
    MJSpacer(MJSpacer),
    MJText(MJText),
    MJWrapper(MJWrapper),
    Node(Node<MJBodyChild>),
    Text(Text),
}

from_child!(MJBodyChild, Comment);
from_child!(MJBodyChild, MJAccordion);
from_child!(MJBodyChild, MJButton);
from_child!(MJBodyChild, MJCarousel);
from_child!(MJBodyChild, MJColumn);
from_child!(MJBodyChild, MJDivider);
from_child!(MJBodyChild, MJGroup);
from_child!(MJBodyChild, MJHero);
from_child!(MJBodyChild, MJImage);
from_child!(MJBodyChild, MJNavbar);
from_child!(MJBodyChild, MJRaw);
from_child!(MJBodyChild, MJSection);
from_child!(MJBodyChild, MJSocial);
from_child!(MJBodyChild, MJSpacer);
from_child!(MJBodyChild, MJText);
from_child!(MJBodyChild, MJWrapper);
from_child!(MJBodyChild, Text);

macro_rules! inner {
    ($elt:ident) => {
        match $elt {
            Self::Comment(elt) => elt,
            Self::MJAccordion(elt) => elt,
            Self::MJButton(elt) => elt,
            Self::MJCarousel(elt) => elt,
            Self::MJColumn(elt) => elt,
            Self::MJDivider(elt) => elt,
            Self::MJGroup(elt) => elt,
            Self::MJHero(elt) => elt,
            Self::MJImage(elt) => elt,
            Self::MJNavbar(elt) => elt,
            Self::MJRaw(elt) => elt,
            Self::MJSection(elt) => elt,
            Self::MJSocial(elt) => elt,
            Self::MJSpacer(elt) => elt,
            Self::MJText(elt) => elt,
            Self::MJWrapper(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    };
}

impl From<Node<MJBodyChild>> for MJBodyChild {
    fn from(value: Node<MJBodyChild>) -> Self {
        Self::Node(value)
    }
}

impl MJBodyChild {
    #[cfg(feature = "render")]
    pub fn as_renderable<'r, 'e: 'r, 'h: 'r>(&'e self) -> &'e (dyn Renderable<'r, 'e, 'h> + 'e) {
        inner!(self)
    }

    #[cfg(feature = "print")]
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        inner!(self)
    }
}

#[cfg(feature = "print")]
impl Print for MJBodyChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}

#[cfg(feature = "parse")]
impl Parsable for MJBodyChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MJAccordion::parse(tag, tokenizer)?.into()),
            MJ_BUTTON => Ok(MJButton::parse(tag, tokenizer)?.into()),
            MJ_CAROUSEL => Ok(MJCarousel::parse(tag, tokenizer)?.into()),
            MJ_COLUMN => Ok(MJColumn::parse(tag, tokenizer)?.into()),
            MJ_DIVIDER => Ok(MJDivider::parse(tag, tokenizer)?.into()),
            MJ_GROUP => Ok(MJGroup::parse(tag, tokenizer)?.into()),
            MJ_HERO => Ok(MJHero::parse(tag, tokenizer)?.into()),
            MJ_IMAGE => Ok(MJImage::parse(tag, tokenizer)?.into()),
            MJ_NAVBAR => Ok(MJNavbar::parse(tag, tokenizer)?.into()),
            MJ_RAW => Ok(MJRaw::parse(tag, tokenizer)?.into()),
            MJ_SECTION => Ok(MJSection::parse(tag, tokenizer)?.into()),
            MJ_SOCIAL => Ok(MJSocial::parse(tag, tokenizer)?.into()),
            MJ_SPACER => Ok(MJSpacer::parse(tag, tokenizer)?.into()),
            MJ_TEXT => Ok(MJText::parse(tag, tokenizer)?.into()),
            MJ_WRAPPER => Ok(MJWrapper::parse(tag, tokenizer)?.into()),
            _ => Ok(Node::<MJBodyChild>::parse(tag, tokenizer)?.into()),
            // _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJBodyChild {
    fn is_raw(&self) -> bool {
        self.as_renderable().is_raw()
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        self.as_renderable().renderer(header)
    }
}
