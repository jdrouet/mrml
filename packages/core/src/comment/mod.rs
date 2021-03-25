mod print;
mod render;

#[derive(Debug)]
pub struct Comment(String);

impl From<String> for Comment {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for Comment {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
