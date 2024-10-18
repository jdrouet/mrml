use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-spacer";

pub struct MjSpacerTag;

impl StaticTag for MjSpacerTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjSpacer = Component<PhantomData<MjSpacerTag>, crate::prelude::AttributeMap, ()>;
