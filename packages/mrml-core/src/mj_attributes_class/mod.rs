use std::marker::PhantomData;

use crate::prelude::{hash::Map, Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-class";

pub struct MjAttributesClassTag;

impl StaticTag for MjAttributesClassTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjAttributesClassAttributes {
    pub name: String,
    #[cfg_attr(feature = "json", serde(flatten))]
    pub others: Map<String, String>,
}

pub type MjAttributesClass =
    Component<PhantomData<MjAttributesClassTag>, MjAttributesClassAttributes, ()>;

#[cfg(test)]
impl MjAttributesClassAttributes {
    #[inline]
    fn new(name: String) -> Self {
        Self {
            name,
            others: Map::default(),
        }
    }
}
