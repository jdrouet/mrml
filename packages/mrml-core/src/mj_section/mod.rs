use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[cfg(feature = "render")]
pub use render::{SectionLikeRender, WithMjSectionBackground};

pub const NAME: &str = "mj-section";

pub struct MjSectionTag;

impl StaticTag for MjSectionTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjSection = Component<PhantomData<MjSectionTag>, Map<String, String>, Vec<MjBodyChild>>;
