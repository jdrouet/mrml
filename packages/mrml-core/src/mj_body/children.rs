#[cfg(feature = "render")]
use std::cell::RefCell;
#[cfg(feature = "render")]
use std::rc::Rc;

use crate::comment::Comment;
use crate::mj_accordion::MjAccordion;
use crate::mj_button::MjButton;
use crate::mj_carousel::MjCarousel;
use crate::mj_column::MjColumn;
use crate::mj_divider::MjDivider;
use crate::mj_group::MjGroup;
use crate::mj_hero::MjHero;
use crate::mj_image::MjImage;
use crate::mj_include::body::MjIncludeBody;
use crate::mj_navbar::MjNavbar;
use crate::mj_raw::MjRaw;
use crate::mj_section::MjSection;
use crate::mj_social::MjSocial;
use crate::mj_spacer::MjSpacer;
use crate::mj_table::MjTable;
use crate::mj_text::MjText;
use crate::mj_wrapper::MjWrapper;
use crate::node::Node;
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
use crate::text::Text;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjBodyChild {
    Comment(Comment),
    MjAccordion(MjAccordion),
    MjButton(MjButton),
    MjCarousel(MjCarousel),
    MjColumn(MjColumn),
    MjDivider(MjDivider),
    MjGroup(MjGroup),
    MjHero(MjHero),
    MjInclude(MjIncludeBody),
    MjImage(MjImage),
    MjNavbar(MjNavbar),
    MjRaw(MjRaw),
    MjSection(MjSection),
    MjSocial(MjSocial),
    MjSpacer(MjSpacer),
    MjTable(MjTable),
    MjText(MjText),
    MjWrapper(MjWrapper),
    Node(Node<MjBodyChild>),
    Text(Text),
}

impl MjBodyChild {
    #[cfg(feature = "render")]
    pub fn as_renderable<'r, 'e: 'r, 'h: 'r>(&'e self) -> &'e (dyn Renderable<'r, 'e, 'h> + 'e) {
        match self {
            Self::Comment(elt) => elt,
            Self::MjAccordion(elt) => elt,
            Self::MjButton(elt) => elt,
            Self::MjCarousel(elt) => elt,
            Self::MjColumn(elt) => elt,
            Self::MjDivider(elt) => elt,
            Self::MjGroup(elt) => elt,
            Self::MjHero(elt) => elt,
            Self::MjInclude(elt) => elt,
            Self::MjImage(elt) => elt,
            Self::MjNavbar(elt) => elt,
            Self::MjRaw(elt) => elt,
            Self::MjSection(elt) => elt,
            Self::MjSocial(elt) => elt,
            Self::MjSpacer(elt) => elt,
            Self::MjTable(elt) => elt,
            Self::MjText(elt) => elt,
            Self::MjWrapper(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjBodyChild {
    fn is_raw(&self) -> bool {
        self.as_renderable().is_raw()
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        self.as_renderable().renderer(header)
    }
}
