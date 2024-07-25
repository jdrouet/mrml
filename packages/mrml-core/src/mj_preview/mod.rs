use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-preview";

pub struct MjPreviewTag;

impl StaticTag for MjPreviewTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjPreview = Component<PhantomData<MjPreviewTag>, (), String>;

impl MjPreview {
    pub fn content(&self) -> &str {
        &self.children
    }
}

impl From<String> for MjPreview {
    fn from(children: String) -> Self {
        Self::new((), children)
    }
}

impl From<&str> for MjPreview {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
