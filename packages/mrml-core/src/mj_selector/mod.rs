use std::marker::PhantomData;

use crate::{
    mj_html_attribute::MjHtmlAttribute,
    prelude::{Component, StaticTag},
};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-selector";

pub struct MjSelectorTag;

impl StaticTag for MjSelectorTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjSelectorAttributes {
    pub path: String,
}

pub type MjSelector =
    Component<PhantomData<MjSelectorTag>, MjSelectorAttributes, Vec<MjHtmlAttribute>>;
