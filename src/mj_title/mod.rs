#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-title";

#[derive(Debug, Default)]
pub struct MJTitle {
    children: String,
}

impl MJTitle {
    pub fn content(&self) -> &str {
        &self.children
    }
}

impl From<String> for MJTitle {
    fn from(children: String) -> Self {
        Self { children }
    }
}

impl From<&str> for MJTitle {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
