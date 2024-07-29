use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-breakpoint";

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjBreakpointAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "String::is_empty"))]
    pub width: String,
}

pub struct MjBreakpointTag;

impl StaticTag for MjBreakpointTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjBreakpoint = Component<PhantomData<MjBreakpointTag>, MjBreakpointAttributes, ()>;

impl MjBreakpoint {
    pub fn value(&self) -> &str {
        &self.attributes.width
    }
}
