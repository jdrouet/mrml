use std::fmt::Write;

use super::{Classes, Styles};

#[derive(Debug, Default)]
pub(crate) struct RenderBuffer {
    inner: String,
}

impl std::fmt::Write for RenderBuffer {
    #[inline]
    fn write_fmt(&mut self, args: std::fmt::Arguments<'_>) -> std::fmt::Result {
        self.inner.write_fmt(args)
    }

    #[inline]
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.inner.write_str(s)
    }

    #[inline]
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        self.inner.write_char(c)
    }
}

pub(crate) struct RenderAttribute<N, V>(N, V);

impl<'a> std::fmt::Display for RenderAttribute<&'a str, &'a str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.0, self.1)
    }
}

impl<'a> std::fmt::Display for RenderAttribute<&'a str, &'a Classes<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.0, self.1)
    }
}

impl<'a> std::fmt::Display for RenderAttribute<&'a str, &'a Styles<'a>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.0, self.1)
    }
}

impl<'a> std::fmt::Display for RenderAttribute<&'a str, Option<&'a str>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.1 {
            Some(ref value) => write!(f, "{}={value:?}", self.0),
            None => write!(f, "{}", self.0),
        }
    }
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

    #[inline]
    pub fn push_attribute<N, V>(&mut self, key: N, value: V) -> std::fmt::Result
    where
        RenderAttribute<N, V>: std::fmt::Display,
    {
        write!(&mut self.inner, " {}", RenderAttribute(key, value))
    }

    #[inline]
    pub fn open_tag(&mut self, tag: &str) {
        self.inner.push('<');
        self.inner.push_str(tag);
    }

    #[inline]
    pub fn closed_tag(&mut self) {
        self.inner.push_str(" />");
    }

    #[inline]
    pub fn close_tag(&mut self) {
        self.inner.push('>');
    }

    #[inline]
    pub fn end_tag(&mut self, tag: &str) {
        self.inner.push_str("</");
        self.inner.push_str(tag);
        self.inner.push('>');
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
