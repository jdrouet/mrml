#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};
use crate::text::Text;

pub const NAME: &str = "mj-accordion-title";

pub struct MjAccordionTitleTag;

impl StaticTag for MjAccordionTitleTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjAccordionTitle =
    Component<PhantomData<MjAccordionTitleTag>, crate::prelude::AttributeMap, Vec<Text>>;
