#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[derive(Debug, Default)]
pub struct Text(String);

impl Text {
    pub fn inner_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<V: Into<String>> From<V> for Text {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}
