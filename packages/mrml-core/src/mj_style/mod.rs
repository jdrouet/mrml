#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-style";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjStyleAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub inline: Option<String>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjStyle {
    pub attributes: MjStyleAttributes,
    pub children: String,
}

impl MjStyle {
    pub fn is_inline(&self) -> bool {
        matches!(self.attributes.inline.as_deref(), Some("inline"))
    }

    pub fn children(&self) -> &str {
        &self.children
    }
}

impl From<String> for MjStyle {
    fn from(children: String) -> Self {
        Self {
            attributes: MjStyleAttributes::default(),
            children,
        }
    }
}

impl From<&str> for MjStyle {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
