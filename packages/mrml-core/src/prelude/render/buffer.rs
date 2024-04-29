#[derive(Debug, Default)]
pub struct RenderBuffer {
    inner: String,
}

impl RenderBuffer {
    #[inline]
    pub fn push_str(&mut self, value: &str) {
        self.inner.push_str(value);
    }

    #[inline]
    pub fn push(&mut self, value: char) {
        self.inner.push(value);
    }
}

impl AsRef<str> for RenderBuffer {
    fn as_ref(&self) -> &str {
        self.inner.as_str()
    }
}

impl From<RenderBuffer> for String {
    fn from(value: RenderBuffer) -> Self {
        value.inner
    }
}
