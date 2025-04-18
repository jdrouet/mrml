#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[derive(Clone, Debug, Default)]
pub struct ConditionalComment(String);

impl ConditionalComment {
    pub fn inner_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for ConditionalComment {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl<V: Into<String>> From<V> for ConditionalComment {
    fn from(value: V) -> Self {
        Self(value.into())
    }
}
