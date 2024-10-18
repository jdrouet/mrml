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

pub use children::MjSocialChild;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-social";

pub struct MjSocialTag;

impl StaticTag for MjSocialTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjSocial =
    Component<PhantomData<MjSocialTag>, crate::prelude::AttributeMap, Vec<MjSocialChild>>;
