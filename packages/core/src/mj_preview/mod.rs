mod parse;
mod print;

pub const NAME: &str = "mj-preview";

#[derive(Debug, Default)]
pub struct MJPreview(String);

impl From<String> for MJPreview {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for MJPreview {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
