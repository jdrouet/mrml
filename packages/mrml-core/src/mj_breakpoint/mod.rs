#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-breakpoint";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjBreakpointAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "String::is_empty"))]
    pub width: String,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjBreakpoint {
    pub attributes: MjBreakpointAttributes,
}

impl MjBreakpoint {
    pub fn value(&self) -> &str {
        &self.attributes.width
    }
}
