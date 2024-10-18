use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
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

pub type MjImage = Component<PhantomData<MjImageTag>, crate::prelude::AttributeMap, ()>;
