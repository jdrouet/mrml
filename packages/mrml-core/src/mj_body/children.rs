use crate::comment::Comment;
use crate::from_child;
use crate::mj_accordion::MJAccordion;
use crate::mj_button::MJButton;
use crate::mj_carousel::MJCarousel;
use crate::mj_column::MJColumn;
use crate::mj_divider::MJDivider;
use crate::mj_group::MJGroup;
use crate::mj_hero::MJHero;
use crate::mj_image::MJImage;
use crate::mj_navbar::MJNavbar;
use crate::mj_raw::MJRaw;
use crate::mj_section::MJSection;
use crate::mj_social::MJSocial;
use crate::mj_spacer::MJSpacer;
use crate::mj_table::MJTable;
use crate::mj_text::MJText;
use crate::mj_wrapper::MJWrapper;
use crate::node::Node;
#[cfg(feature = "print")]
use crate::prelude::print::Print;
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
use crate::text::Text;
#[cfg(feature = "render")]
use std::cell::RefCell;
#[cfg(feature = "render")]
use std::rc::Rc;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
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
    MJTable(MJTable),
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
from_child!(MJBodyChild, MJTable);
from_child!(MJBodyChild, MJText);
from_child!(MJBodyChild, MJWrapper);
from_child!(MJBodyChild, Text);

#[cfg(any(feature = "render", feature = "print"))]
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
            Self::MJTable(elt) => elt,
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

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJBodyChild {
    fn is_raw(&self) -> bool {
        self.as_renderable().is_raw()
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        self.as_renderable().renderer(header)
    }
}
