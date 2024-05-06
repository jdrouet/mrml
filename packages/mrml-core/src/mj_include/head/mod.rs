#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[cfg(any(feature = "print", feature = "json"))]
use super::NAME;

#[derive(Debug)]
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

#[derive(Debug, Default)]
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

impl MjIncludeHeadAttributes {
    pub fn is_empty(&self) -> bool {
        false
    }
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjIncludeHead {
    pub attributes: MjIncludeHeadAttributes,
    pub children: Vec<MjIncludeHeadChild>,
}
