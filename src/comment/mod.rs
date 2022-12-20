#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[derive(Debug, Default)]
pub struct Comment {
    children: String,
}

impl From<String> for Comment {
    fn from(children: String) -> Self {
        Self { children }
    }
}

impl From<&str> for Comment {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
