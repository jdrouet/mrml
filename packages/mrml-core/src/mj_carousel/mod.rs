mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

pub use children::MjCarouselChild;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-carousel";

pub struct MjCarouselTag;

impl StaticTag for MjCarouselTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjCarousel =
    Component<PhantomData<MjCarouselTag>, crate::prelude::AttributeMap, Vec<MjCarouselChild>>;
