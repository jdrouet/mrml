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

const START_CONDITIONAL_TAG: &str = "<!--[if mso | IE]>";
const START_MSO_CONDITIONAL_TAG: &str = "<!--[if mso]>";
const END_CONDITIONAL_TAG: &str = "<![endif]-->";
const START_NEGATION_CONDITIONAL_TAG: &str = "<!--[if !mso | IE]><!-->";
const START_MSO_NEGATION_CONDITIONAL_TAG: &str = "<!--[if !mso]><!-->";
const END_NEGATION_CONDITIONAL_TAG: &str = "<!--<![endif]-->";

impl RenderBuffer {
    #[inline]
    pub fn start_conditional_tag(&mut self) {
        self.inner.push_str(START_CONDITIONAL_TAG);
    }

    #[inline]
    pub fn start_negation_conditional_tag(&mut self) {
        self.inner.push_str(START_NEGATION_CONDITIONAL_TAG);
    }

    #[inline]
    pub fn start_mso_conditional_tag(&mut self) {
        self.inner.push_str(START_MSO_CONDITIONAL_TAG);
    }

    #[inline]
    pub fn start_mso_negation_conditional_tag(&mut self) {
        self.inner.push_str(START_MSO_NEGATION_CONDITIONAL_TAG);
    }

    #[inline]
    pub fn end_conditional_tag(&mut self) {
        self.inner.push_str(END_CONDITIONAL_TAG);
    }

    #[inline]
    pub fn end_negation_conditional_tag(&mut self) {
        self.inner.push_str(END_NEGATION_CONDITIONAL_TAG);
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
