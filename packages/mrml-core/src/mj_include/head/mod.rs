#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "render", derive(enum_as_inner::EnumAsInner))]
pub enum MjIncludeHeadChild {
    Comment(crate::comment::Comment),
    MjAttributes(crate::mj_attributes::MjAttributes),
    MjBreakpoint(crate::mj_breakpoint::MjBreakpoint),
    MjFont(crate::mj_font::MjFont),
    MjPreview(crate::mj_preview::MjPreview),
    MjRaw(crate::mj_raw::MjRaw),
    MjStyle(crate::mj_style::MjStyle),
    MjTitle(crate::mj_title::MjTitle),
    Text(crate::text::Text),
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(rename_all = "snake_case"))]
pub enum MjIncludeHeadKind {
    Mjml,
    Html,
    Css { inline: bool },
}

impl AsRef<str> for MjIncludeHeadKind {
    fn as_ref(&self) -> &str {
        match self {
            Self::Html => "html",
            Self::Mjml => "mjml",
            Self::Css { inline: _ } => "css",
        }
    }
}

#[cfg(feature = "json")]
impl MjIncludeHeadKind {
    fn is_default(&self) -> bool {
        matches!(self, Self::Mjml)
    }
}

impl Default for MjIncludeHeadKind {
    fn default() -> Self {
        Self::Mjml
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
pub struct MjIncludeHeadAttributes {
    pub path: String,
    #[cfg_attr(
        feature = "json",
        serde(
            default,
            rename = "type",
            skip_serializing_if = "MjIncludeHeadKind::is_default"
        )
    )]
    pub kind: MjIncludeHeadKind,
}

#[cfg(test)]
impl MjIncludeHeadAttributes {
    pub fn new<P: Into<String>>(path: P) -> Self {
        Self {
            path: path.into(),
            kind: MjIncludeHeadKind::default(),
        }
    }

    pub fn with_kind(mut self, kind: MjIncludeHeadKind) -> Self {
        self.kind = kind;
        self
    }
}

pub struct MjIncludeHeadTag;

impl StaticTag for MjIncludeHeadTag {
    fn static_tag() -> &'static str {
        super::NAME
    }
}

pub type MjIncludeHeadInner =
    Component<PhantomData<MjIncludeHeadTag>, MjIncludeHeadAttributes, Vec<MjIncludeHeadChild>>;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(transparent))]
pub struct MjIncludeHead(pub MjIncludeHeadInner);

impl MjIncludeHead {
    #[inline]
    pub fn new(attributes: MjIncludeHeadAttributes, children: Vec<MjIncludeHeadChild>) -> Self {
        Self(MjIncludeHeadInner::new(attributes, children))
    }
}
