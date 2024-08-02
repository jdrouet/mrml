use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-column";

pub struct MjColumnTag;

impl StaticTag for MjColumnTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjColumn = Component<PhantomData<MjColumnTag>, Map<String, String>, Vec<MjBodyChild>>;
