#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-preview";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjPreview {
    pub children: String,
}

impl MjPreview {
    pub fn content(&self) -> &str {
        &self.children
    }
}

impl From<String> for MjPreview {
    fn from(children: String) -> Self {
        Self { children }
    }
}

impl From<&str> for MjPreview {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
