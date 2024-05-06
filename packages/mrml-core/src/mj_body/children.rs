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
use crate::prelude::render::{Render, RenderContext, Renderable};
use crate::text::Text;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
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

#[cfg(feature = "render")]
impl<'render, 'root: 'render> Renderable<'render, 'root> for MjBodyChild {
    fn is_raw(&self) -> bool {
        match self {
            Self::Comment(elt) => elt.is_raw(),
            Self::MjAccordion(elt) => elt.is_raw(),
            Self::MjButton(elt) => elt.is_raw(),
            Self::MjCarousel(elt) => elt.is_raw(),
            Self::MjColumn(elt) => elt.is_raw(),
            Self::MjDivider(elt) => elt.is_raw(),
            Self::MjGroup(elt) => elt.is_raw(),
            Self::MjHero(elt) => elt.is_raw(),
            Self::MjInclude(elt) => elt.is_raw(),
            Self::MjImage(elt) => elt.is_raw(),
            Self::MjNavbar(elt) => elt.is_raw(),
            Self::MjRaw(elt) => elt.is_raw(),
            Self::MjSection(elt) => elt.is_raw(),
            Self::MjSocial(elt) => elt.is_raw(),
            Self::MjSpacer(elt) => elt.is_raw(),
            Self::MjTable(elt) => elt.is_raw(),
            Self::MjText(elt) => elt.is_raw(),
            Self::MjWrapper(elt) => elt.is_raw(),
            Self::Node(elt) => elt.is_raw(),
            Self::Text(elt) => elt.is_raw(),
        }
    }

    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        match self {
            Self::Comment(elt) => elt.renderer(context),
            Self::MjAccordion(elt) => elt.renderer(context),
            Self::MjButton(elt) => elt.renderer(context),
            Self::MjCarousel(elt) => elt.renderer(context),
            Self::MjColumn(elt) => elt.renderer(context),
            Self::MjDivider(elt) => elt.renderer(context),
            Self::MjGroup(elt) => elt.renderer(context),
            Self::MjHero(elt) => elt.renderer(context),
            Self::MjInclude(elt) => elt.renderer(context),
            Self::MjImage(elt) => elt.renderer(context),
            Self::MjNavbar(elt) => elt.renderer(context),
            Self::MjRaw(elt) => elt.renderer(context),
            Self::MjSection(elt) => elt.renderer(context),
            Self::MjSocial(elt) => elt.renderer(context),
            Self::MjSpacer(elt) => elt.renderer(context),
            Self::MjTable(elt) => elt.renderer(context),
            Self::MjText(elt) => elt.renderer(context),
            Self::MjWrapper(elt) => elt.renderer(context),
            Self::Node(elt) => elt.renderer(context),
            Self::Text(elt) => elt.renderer(context),
        }
    }
}
