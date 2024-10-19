use std::marker::PhantomData;

use crate::mj_raw::MjRawChild;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-text";

pub struct MjTextTag;

impl StaticTag for MjTextTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjText = Component<PhantomData<MjTextTag>, crate::prelude::AttributeMap, Vec<MjRawChild>>;
