#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-carousel-image";

pub struct MjCarouselImageTag;

impl StaticTag for MjCarouselImageTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjCarouselImage =
    Component<PhantomData<MjCarouselImageTag>, crate::prelude::AttributeMap, ()>;
