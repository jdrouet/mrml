use std::marker::PhantomData;

use crate::prelude::{hash::Map, Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-image";

pub struct MjImageTag;

impl StaticTag for MjImageTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjImage = Component<PhantomData<MjImageTag>, Map<String, String>, ()>;
