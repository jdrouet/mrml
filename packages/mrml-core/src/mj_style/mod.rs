use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-style";

/// Specifies how CSS styles should be applied in the rendered output.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StyleInlineMode {
    /// CSS styles will be output in a `<style>` tag in the document head.
    #[default]
    StyleTag,
    /// CSS styles will be processed by css-inline and merged into element style
    /// attributes.
    Inline,
}

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
    pub fn inline_mode(&self) -> StyleInlineMode {
        if matches!(self.attributes.inline.as_deref(), Some("inline")) {
            StyleInlineMode::Inline
        } else {
            StyleInlineMode::StyleTag
        }
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
