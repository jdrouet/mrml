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

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
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

impl ToString for MjIncludeHeadKind {
    fn to_string(&self) -> String {
        match self {
            Self::Html => "html".to_string(),
            Self::Mjml => "mjml".to_string(),
            Self::Css { inline: _ } => "css".to_string(),
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
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME", children = false))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjIncludeHead {
    pub attributes: MjIncludeHeadAttributes,
    pub children: Vec<MjIncludeHeadChild>,
}
