use std::rc::Rc;

use xmlparser::{StrSpan, Token, Tokenizer};

use self::loader::IncludeLoaderError;

#[cfg(feature = "http-loader-base")]
pub mod http_loader;
pub mod loader;
#[cfg(feature = "local-loader")]
pub mod local_loader;
pub mod memory_loader;
pub mod noop_loader;

#[macro_export]
macro_rules! parse_attribute {
    () => {
        fn parse_attribute<'a>(
            &mut self,
            name: xmlparser::StrSpan<'a>,
            value: xmlparser::StrSpan<'a>,
        ) -> Result<(), Error> {
            self.attributes.insert(name.to_string(), value.to_string());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_child {
    ($child_parser:ident) => {
        fn parse_child_element<'a>(
            &mut self,
            tag: xmlparser::StrSpan<'a>,
            tokenizer: &mut xmlparser::Tokenizer<'a>,
        ) -> Result<(), Error> {
            self.children
                .push($child_parser::parse(tag, tokenizer, self.opts.clone())?);
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_comment {
    () => {
        fn parse_child_comment(&mut self, value: xmlparser::StrSpan) -> Result<(), Error> {
            self.children
                .push($crate::comment::Comment::from(value.as_str()).into());
            Ok(())
        }
    };
}

#[macro_export]
macro_rules! parse_text {
    () => {
        fn parse_child_text(&mut self, value: xmlparser::StrSpan) -> Result<(), Error> {
            self.children
                .push($crate::text::Text::from(value.as_str()).into());
            Ok(())
        }
    };
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected attribute at position {0}")]
    UnexpectedAttribute(usize),
    #[error("unexpected element at position {0}")]
    UnexpectedElement(usize),
    #[error("unexpected comment at position {0}")]
    UnexpectedComment(usize),
    #[error("unexpected text at position {0}")]
    UnexpectedText(usize),
    #[error("missing attribute {0}")]
    MissingAttribute(&'static str),
    #[error("invalid element: {0}")]
    InvalidElement(String),
    #[error("invalid format at position {0}")]
    InvalidFormat(usize, usize),
    #[error("unexpected end of stream")]
    EndOfStream,
    /// The input string should be smaller than 4GiB.
    #[error("size limit reached")]
    SizeLimit,
    /// Errors detected by the `xmlparser` crate.
    #[error("unable to load included template")]
    ParserError(#[from] xmlparser::Error),
    /// The Mjml document must have at least one element.
    #[error("no root node found")]
    NoRootNode,
    #[error("unable to load included template")]
    IncludeLoaderError(#[from] IncludeLoaderError),
}

impl Error {
    fn invalid_format((start, end): (usize, usize)) -> Self {
        Error::InvalidFormat(start, end)
    }
}

pub(crate) fn next_token<'a>(tokenizer: &mut Tokenizer<'a>) -> Result<Token<'a>, Error> {
    if let Some(token) = tokenizer.next() {
        Ok(token?)
    } else {
        Err(Error::EndOfStream)
    }
}

pub(crate) fn is_element_start<'a>(token: &'a Token<'a>) -> Option<&'a StrSpan<'a>> {
    match token {
        Token::ElementStart { local, .. } => Some(local),
        _ => None,
    }
}

pub(crate) fn into_element_start<'a>(token: &'a Token<'a>) -> Result<&'a StrSpan<'a>, Error> {
    match token {
        Token::ElementStart { local, .. } => Ok(local),
        other => Err(Error::invalid_format(get_span(&other))),
    }
}

fn get_span<'a>(token: &Token<'a>) -> (usize, usize) {
    match token {
        Token::Attribute { span, .. }
        | Token::Cdata { span, .. }
        | Token::Comment { span, .. }
        | Token::Declaration { span, .. }
        | Token::DtdEnd { span }
        | Token::DtdStart { span, .. }
        | Token::ElementEnd { span, .. }
        | Token::ElementStart { span, .. }
        | Token::EmptyDtd { span, .. }
        | Token::EntityDeclaration { span, .. }
        | Token::ProcessingInstruction { span, .. } => (span.start(), span.end()),
        Token::Text { text } => (text.start(), text.end()),
    }
}

pub trait Parser: Sized {
    type Output;

    fn build(self) -> Result<Self::Output, Error>;

    /// for elements like <br> or <meta>, they have no closing elements and
    /// the parser should not check for children
    fn should_ignore_children(&self) -> bool {
        false
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, _value: StrSpan<'a>) -> Result<(), Error> {
        Err(Error::UnexpectedAttribute(name.start()))
    }

    fn parse_children(&mut self, tokenizer: &mut Tokenizer<'_>) -> Result<(), Error> {
        loop {
            let token = next_token(tokenizer)?;
            match token {
                Token::Comment { text, span: _ } => {
                    self.parse_child_comment(text)?;
                }
                Token::Text { text } => {
                    if !text.trim().is_empty() {
                        self.parse_child_text(text)?;
                    }
                }
                Token::ElementStart {
                    prefix: _,
                    local,
                    span: _,
                } => {
                    self.parse_child_element(local, tokenizer)?;
                }
                other => {
                    return Err(Error::invalid_format(get_span(&other)));
                }
            };
        }
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        _tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        Err(Error::UnexpectedElement(tag.start()))
    }

    fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
        Err(Error::UnexpectedComment(value.start()))
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        Err(Error::UnexpectedText(value.start()))
    }

    fn parse(mut self, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        loop {
            let token = next_token(tokenizer)?;
            match token {
                Token::Attribute {
                    prefix: _,
                    local,
                    value,
                    span: _,
                } => {
                    self.parse_attribute(local, value)?;
                }
                Token::ElementEnd { end, span } => {
                    match end {
                        xmlparser::ElementEnd::Empty => {
                            return Ok(self);
                        }
                        xmlparser::ElementEnd::Open => {
                            if !self.should_ignore_children() {
                                self.parse_children(tokenizer)?;
                            }
                            return Ok(self);
                        }
                        // unexpected
                        _ => return Err(Error::InvalidFormat(span.start(), span.end())),
                    }
                }
                other => {
                    return Err(Error::invalid_format(get_span(&other)));
                }
            };
        }
    }
}

pub trait Parsable: Sized {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error>;
}

#[derive(Debug)]
pub struct ParserOptions {
    pub include_loader: Box<dyn loader::IncludeLoader>,
}

#[allow(clippy::box_default)]
impl Default for ParserOptions {
    fn default() -> Self {
        Self {
            include_loader: Box::new(noop_loader::NoopIncludeLoader),
        }
    }
}
