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

pub use children::MjNavbarChild;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-navbar";

pub struct MjNavbarTag;

impl StaticTag for MjNavbarTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjNavbar =
    Component<PhantomData<MjNavbarTag>, crate::prelude::AttributeMap, Vec<MjNavbarChild>>;
