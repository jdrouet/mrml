use roxmltree::Node;
use std::collections::HashMap;

pub mod mj_accordion;
pub mod mj_body;
pub mod mj_button;
pub mod mj_carousel;
pub mod mj_column;
pub mod mj_divider;
pub mod mj_group;
pub mod mj_hero;
pub mod mj_image;
pub mod mj_navbar;
pub mod mj_raw;
pub mod mj_section;
pub mod mj_social;
pub mod mj_spacer;
pub mod mj_table;
pub mod mj_text;
pub mod mj_wrapper;
pub mod prelude;
pub mod raw;

use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::attributes::Attributes;
use crate::util::{Context, Header, Size, Tag};
use prelude::BodyComponent;

#[derive(Clone, Debug)]
pub enum BodyElement {
    MJAccordion(mj_accordion::MJAccordion),
    MJAccordionElement(mj_accordion::MJAccordionElement),
    MJButton(mj_button::MJButton),
    MJCarousel(mj_carousel::MJCarousel),
    MJCarouselImage(mj_carousel::MJCarouselImage),
    MJColumn(mj_column::MJColumn),
    MJDivider(mj_divider::MJDivider),
    MJGroup(mj_group::MJGroup),
    MJHero(mj_hero::MJHero),
    MJImage(mj_image::MJImage),
    MJNavbar(mj_navbar::MJNavbar),
    MJNavbarLink(mj_navbar::MJNavbarLink),
    MJRaw(mj_raw::MJRaw),
    MJSection(mj_section::MJSection),
    MJSocial(mj_social::MJSocial),
    MJSocialElement(mj_social::MJSocialElement),
    MJSpacer(mj_spacer::MJSpacer),
    MJTable(mj_table::MJTable),
    MJText(mj_text::MJText),
    MJWrapper(mj_wrapper::MJWrapper),
    Raw(raw::RawElement),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            BodyElement::MJAccordion(item) => item.$func($($args)*),
            BodyElement::MJAccordionElement(item) => item.$func($($args)*),
            BodyElement::MJButton(item) => item.$func($($args)*),
            BodyElement::MJCarousel(item) => item.$func($($args)*),
            BodyElement::MJCarouselImage(item) => item.$func($($args)*),
            BodyElement::MJColumn(item) => item.$func($($args)*),
            BodyElement::MJDivider(item) => item.$func($($args)*),
            BodyElement::MJGroup(item) => item.$func($($args)*),
            BodyElement::MJHero(item) => item.$func($($args)*),
            BodyElement::MJImage(item) => item.$func($($args)*),
            BodyElement::MJNavbar(item) => item.$func($($args)*),
            BodyElement::MJNavbarLink(item) => item.$func($($args)*),
            BodyElement::MJRaw(item) => item.$func($($args)*),
            BodyElement::MJSection(item) => item.$func($($args)*),
            BodyElement::MJSocial(item) => item.$func($($args)*),
            BodyElement::MJSocialElement(item) => item.$func($($args)*),
            BodyElement::MJSpacer(item) => item.$func($($args)*),
            BodyElement::MJTable(item) => item.$func($($args)*),
            BodyElement::MJText(item) => item.$func($($args)*),
            BodyElement::MJWrapper(item) => item.$func($($args)*),
            BodyElement::Raw(item) => item.$func($($args)*),
        }
    };
}

impl Component for BodyElement {
    fn update_header(&self, header: &mut Header) {
        apply_fn!(self, update_header(header))
    }

    fn context(&self) -> Option<&Context> {
        apply_fn!(self, context())
    }

    fn set_context(&mut self, ctx: Context) {
        apply_fn!(self, set_context(ctx))
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        apply_fn!(self, render(header))
    }
}

impl ComponentWithAttributes for BodyElement {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        apply_fn!(self, attributes())
    }

    fn get_attribute(&self, key: &str) -> Option<&String> {
        apply_fn!(self, get_attribute(key))
    }
}

impl BodyComponent for BodyElement {
    fn set_style(&self, key: &str, tag: Tag) -> Tag {
        apply_fn!(self, set_style(key, tag))
    }

    fn get_width(&self) -> Option<Size> {
        apply_fn!(self, get_width())
    }
}

impl BodyElement {
    pub fn parse<'a, 'b>(
        node: &Node<'a, 'b>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<BodyElement, Error> {
        let res = match node.tag_name().name() {
            "mj-accordion" => {
                BodyElement::MJAccordion(mj_accordion::MJAccordion::parse(node, header)?)
            }
            "mj-button" => BodyElement::MJButton(mj_button::MJButton::parse(node, header)?),
            "mj-carousel" => BodyElement::MJCarousel(mj_carousel::MJCarousel::parse(node, header)?),
            "mj-column" => BodyElement::MJColumn(mj_column::MJColumn::parse(node, header, extra)?),
            "mj-divider" => BodyElement::MJDivider(mj_divider::MJDivider::parse(node, header)?),
            "mj-group" => BodyElement::MJGroup(mj_group::MJGroup::parse(node, header)?),
            "mj-hero" => BodyElement::MJHero(mj_hero::MJHero::parse(node, header)?),
            "mj-image" => BodyElement::MJImage(mj_image::MJImage::parse(node, header)?),
            "mj-navbar" => BodyElement::MJNavbar(mj_navbar::MJNavbar::parse(node, header)?),
            "mj-raw" => BodyElement::MJRaw(mj_raw::MJRaw::parse(node, header)?),
            "mj-section" => BodyElement::MJSection(mj_section::MJSection::parse(node, header)?),
            "mj-social" => BodyElement::MJSocial(mj_social::MJSocial::parse(node, header)?),
            "mj-spacer" => BodyElement::MJSpacer(mj_spacer::MJSpacer::parse(node, header)?),
            "mj-table" => BodyElement::MJTable(mj_table::MJTable::parse(node, header)?),
            "mj-text" => BodyElement::MJText(mj_text::MJText::parse(node, header)?),
            "mj-wrapper" => BodyElement::MJWrapper(mj_wrapper::MJWrapper::parse(node, header)?),
            _ => BodyElement::Raw(raw::RawElement::parse(node, header)?),
        };
        Ok(res)
    }

    pub fn is_raw(&self) -> bool {
        match self {
            BodyElement::Raw(_) => true,
            _ => false,
        }
    }
}
