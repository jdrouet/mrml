use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-title";

pub struct MjTitleTag;

impl StaticTag for MjTitleTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjTitle = Component<PhantomData<MjTitleTag>, (), String>;

impl MjTitle {
    pub fn content(&self) -> &str {
        &self.children
    }
}

impl From<String> for MjTitle {
    fn from(children: String) -> Self {
        Self::new((), children)
    }
}

impl From<&str> for MjTitle {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
