//! Module containing the trait for implementing an [`IncludeLoader`](crate::prelude::parse::loader::IncludeLoader).

use super::ParserOptions;
use crate::{
    comment::Comment,
    prelude::parse::{next_token, Error, Parsable},
    text::Text,
};
use std::io::ErrorKind;
use std::rc::Rc;
use xmlparser::Token;

#[derive(Debug)]
pub struct IncludeLoaderError {
    pub path: String,
    pub reason: ErrorKind,
    pub cause: Option<Box<dyn std::error::Error>>,
}

impl IncludeLoaderError {
    pub fn new<P: ToString>(path: P, reason: ErrorKind) -> Self {
        Self {
            path: path.to_string(),
            reason,
            cause: None,
        }
    }

    pub fn not_found<P: ToString>(path: P) -> Self {
        Self {
            path: path.to_string(),
            reason: ErrorKind::NotFound,
            cause: None,
        }
    }
}

impl std::fmt::Display for IncludeLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to load template {}: {}", self.path, self.reason)
    }
}

impl std::error::Error for IncludeLoaderError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.cause.as_ref().map(|c| c.as_ref())
    }
}

pub trait IncludeLoader: std::fmt::Debug {
    /// This function is used to fetch the included template using the `path` attribute.
    ///
    /// You can have an example of simple resolve function with the [`MemoryIncludeLoader`](crate::prelude::parse::memory_loader::MemoryIncludeLoader).
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
        _ => Err(Error::InvalidFormat),
    }
}
