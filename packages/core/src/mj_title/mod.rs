mod parse;
mod print;

pub const NAME: &str = "mj-title";

#[derive(Debug, Default)]
pub struct MJTitle(String);

impl MJTitle {
    pub fn content(&self) -> &str {
        &self.0
    }
}

impl From<String> for MJTitle {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<&str> for MJTitle {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
