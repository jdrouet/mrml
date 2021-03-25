mod print;
mod render;

#[derive(Debug)]
pub struct Text(String);

impl From<String> for Text {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Text {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
