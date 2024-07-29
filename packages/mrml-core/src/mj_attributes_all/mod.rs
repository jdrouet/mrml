use std::marker::PhantomData;

use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-all";

pub struct MjAttributesAllTag;

impl StaticTag for MjAttributesAllTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjAttributesAll = Component<PhantomData<MjAttributesAllTag>, Map<String, String>, ()>;

impl MjAttributesAll {
    pub fn attributes(&self) -> &Map<String, String> {
        &self.attributes
    }
}
