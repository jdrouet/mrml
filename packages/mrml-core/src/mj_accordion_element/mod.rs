#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;
use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-accordion-element";

pub struct MjAccordionElementTag;

impl StaticTag for MjAccordionElementTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

#[derive(Clone, Debug, Default)]
pub struct MjAccordionElementChildren {
    pub title: Option<MjAccordionTitle>,
    pub text: Option<MjAccordionText>,
}

pub type MjAccordionElement = Component<
    PhantomData<MjAccordionElementTag>,
    crate::prelude::AttributeMap,
    MjAccordionElementChildren,
>;
