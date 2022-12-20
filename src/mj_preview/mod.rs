#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-preview";

#[derive(Debug, Default)]
pub struct MJPreview {
    children: String,
}

impl MJPreview {
    pub fn content(&self) -> &str {
        &self.children
    }
}

impl From<String> for MJPreview {
    fn from(children: String) -> Self {
        Self { children }
    }
}

impl From<&str> for MJPreview {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
