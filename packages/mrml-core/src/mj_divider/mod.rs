use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-divider";

pub struct MjDividerTag;

impl StaticTag for MjDividerTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjDivider = Component<PhantomData<MjDividerTag>, crate::prelude::AttributeMap, ()>;
