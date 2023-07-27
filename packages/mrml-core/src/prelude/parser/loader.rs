//! Module containing the trait for implementing an
//! [`IncludeLoader`](crate::prelude::parser::loader::IncludeLoader).

use std::io::ErrorKind;
use std::rc::Rc;

use xmlparser::Token;

use super::ParserOptions;
use crate::comment::Comment;
use crate::prelude::parser::{next_token, Error, Parsable};
use crate::text::Text;

#[derive(Debug)]
pub struct IncludeLoaderError {
    pub path: String,
    pub reason: ErrorKind,
    pub message: Option<&'static str>,
    pub cause: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}

impl IncludeLoaderError {
    pub fn new<P: ToString>(path: P, reason: ErrorKind) -> Self {
        Self {
            path: path.to_string(),
            reason,
            message: None,
            cause: None,
        }
    }

    pub fn not_found<P: ToString>(path: P) -> Self {
        Self {
            path: path.to_string(),
            reason: ErrorKind::NotFound,
            message: None,
            cause: None,
        }
    }

    pub fn with_message(mut self, message: &'static str) -> Self {
        self.message = Some(message);
        self
    }

    pub fn with_cause(mut self, cause: Box<dyn std::error::Error + Send + Sync + 'static>) -> Self {
        self.cause = Some(cause);
        self
    }
}

impl std::fmt::Display for IncludeLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(msg) = self.message {
            write!(
                f,
                "Unable to load template {}: {} ({})",
                self.path, msg, self.reason
            )
        } else {
            write!(f, "Unable to load template {}: {}", self.path, self.reason)
        }
    }
}

impl std::error::Error for IncludeLoaderError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.cause
            .as_ref()
            .map(|c| c.as_ref() as &dyn std::error::Error)
    }
}

pub trait IncludeLoader: std::fmt::Debug {
    /// This function is used to fetch the included template using the `path`
    /// attribute.
    ///
    /// You can have an example of simple resolve function with the
    /// [`MemoryIncludeLoader`](crate::prelude::parser::memory_loader::MemoryIncludeLoader).
    ///
    fn resolve(&self, path: &str) -> Result<String, IncludeLoaderError>;
}

pub fn parse<T: Parsable + From<Comment> + From<Text>>(
    include: &str,
    opts: Rc<ParserOptions>,
) -> Result<T, Error> {
    let mut tokenizer = xmlparser::Tokenizer::from(include);
    let token = next_token(&mut tokenizer)?;
    match token {
        Token::Comment { text, span: _ } => Ok(Comment::from(text.to_string()).into()),
        Token::Text { text } => Ok(Text::from(text.to_string()).into()),
        Token::ElementStart { local, .. } => T::parse(local, &mut tokenizer, opts),
        other => Err(Error::invalid_format(super::get_span(&other))),
    }
}

#[cfg(test)]
mod tests {
    use std::io::ErrorKind;

    use super::IncludeLoaderError;

    #[test]
    fn should_display_basic() {
        assert_eq!(
            IncludeLoaderError::new("foo.mjml", ErrorKind::NotFound).to_string(),
            "Unable to load template foo.mjml: entity not found",
        );
    }

    #[test]
    fn should_display_with_message() {
        assert_eq!(
            IncludeLoaderError::new("foo.mjml", ErrorKind::NotFound)
                .with_message("oops")
                .to_string(),
            "Unable to load template foo.mjml: oops (entity not found)",
        );
    }

    #[test]
    fn should_display_with_cause() {
        assert_eq!(
            IncludeLoaderError::new("foo.mjml", ErrorKind::NotFound)
                .with_cause(Box::new(IncludeLoaderError::new(
                    "bar.mjml",
                    ErrorKind::InvalidInput
                )))
                .to_string(),
            "Unable to load template foo.mjml: entity not found",
        );
    }
}
