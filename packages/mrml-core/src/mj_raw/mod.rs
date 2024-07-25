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

pub use children::MjRawChild;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-raw";

pub struct MjRawTag;

impl StaticTag for MjRawTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjRaw = Component<PhantomData<MjRawTag>, (), Vec<MjRawChild>>;
