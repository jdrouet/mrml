use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-table";

pub struct MjTableTag;

impl StaticTag for MjTableTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjTable = Component<PhantomData<MjTableTag>, Map<String, String>, Vec<MjBodyChild>>;
