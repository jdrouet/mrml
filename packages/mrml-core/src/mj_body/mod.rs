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

pub use children::MjBodyChild;

use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-body";

pub struct MjBodyTag;

impl StaticTag for MjBodyTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjBody = Component<PhantomData<MjBodyTag>, Map<String, String>, Vec<MjBodyChild>>;
