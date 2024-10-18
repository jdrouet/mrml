use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-wrapper";

pub struct MjWrapperTag;

impl StaticTag for MjWrapperTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjWrapper =
    Component<PhantomData<MjWrapperTag>, crate::prelude::AttributeMap, Vec<MjBodyChild>>;
