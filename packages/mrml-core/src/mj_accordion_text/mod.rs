#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::mj_raw::MjRawChild;
use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-accordion-text";

pub struct MjAccordionTextTag;

impl StaticTag for MjAccordionTextTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjAccordionText =
    Component<PhantomData<MjAccordionTextTag>, Map<String, String>, Vec<MjRawChild>>;
