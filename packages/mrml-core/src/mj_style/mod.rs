use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-style";

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjStyleAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub inline: Option<String>,
}

pub struct MjStyleTag;

impl StaticTag for MjStyleTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjStyle = Component<PhantomData<MjStyleTag>, MjStyleAttributes, String>;

impl MjStyle {
    pub fn is_inline(&self) -> bool {
        matches!(
            self.attributes.inline.as_deref(),
            Some("inline") | Some("true")
        )
    }

    pub fn children(&self) -> &str {
        &self.children
    }
}

impl From<String> for MjStyle {
    fn from(children: String) -> Self {
        Self::new(MjStyleAttributes::default(), children)
    }
}

impl From<&str> for MjStyle {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
