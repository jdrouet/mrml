use std::{convert::TryFrom, sync::Arc};

use xmlparser::{StrSpan, Token, Tokenizer};

use self::loader::IncludeLoaderError;

use super::hash::Map;

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

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected attribute at position {0}")]
    UnexpectedAttribute(usize),
    #[error("unexpected element at position {0}")]
    UnexpectedElement(usize),
    #[error("unexpected comment at position {0}")]
    UnexpectedComment(usize),
    #[error("unexpected text at position {0}")]
    UnexpectedText(usize),
    #[error("unexpected token at position {0}..{1}")]
    UnexpectedToken(usize, usize),
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
    pub(crate) fn unexpected_token((start, end): (usize, usize)) -> Self {
        Error::UnexpectedToken(start, end)
    }
}

fn get_span(token: &Token<'_>) -> (usize, usize) {
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

#[derive(Debug)]
pub enum MrmlToken<'a> {
    Attribute(Attribute<'a>),
    Comment(Comment<'a>),
    ElementClose(ElementClose<'a>),
    ElementEnd(ElementEnd<'a>),
    ElementStart(ElementStart<'a>),
    Text(Text<'a>),
}

impl<'a> TryFrom<Token<'a>> for MrmlToken<'a> {
    type Error = Error;

    fn try_from(value: Token<'a>) -> Result<Self, Self::Error> {
        match value {
            Token::Attribute {
                prefix,
                local,
                value,
                span,
            } => Ok(MrmlToken::Attribute(Attribute {
                prefix,
                local,
                value,
                span,
            })),
            Token::Comment { text, span } => Ok(MrmlToken::Comment(Comment { span, text })),
            Token::ElementEnd {
                end: xmlparser::ElementEnd::Close(prefix, local),
                span,
            } => Ok(MrmlToken::ElementClose(ElementClose {
                span,
                prefix,
                local,
            })),
            Token::ElementEnd {
                end: xmlparser::ElementEnd::Empty,
                span,
            } => Ok(MrmlToken::ElementEnd(ElementEnd { span, empty: true })),
            Token::ElementEnd {
                end: xmlparser::ElementEnd::Open,
                span,
            } => Ok(MrmlToken::ElementEnd(ElementEnd { span, empty: false })),
            Token::ElementStart {
                prefix,
                local,
                span,
            } => Ok(MrmlToken::ElementStart(ElementStart {
                prefix,
                local,
                span,
            })),
            Token::Text { text } => Ok(MrmlToken::Text(Text { text })),
            other => Err(Error::unexpected_token(get_span(&other))),
        }
    }
}

impl<'a> MrmlToken<'a> {
    pub fn range(&self) -> (usize, usize) {
        let span = match self {
            Self::Attribute(item) => item.span,
            Self::Comment(item) => item.span,
            Self::ElementClose(item) => item.span,
            Self::ElementEnd(item) => item.span,
            Self::ElementStart(item) => item.span,
            Self::Text(item) => item.text,
        };
        (span.start(), span.end())
    }
}

#[derive(Debug)]
pub struct Attribute<'a> {
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub value: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub struct Comment<'a> {
    pub span: StrSpan<'a>,
    pub text: StrSpan<'a>,
}

#[derive(Debug)]
pub struct ElementClose<'a> {
    pub span: StrSpan<'a>,
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
}

#[derive(Debug)]
pub struct ElementStart<'a> {
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub struct ElementEnd<'a> {
    pub span: StrSpan<'a>,
    pub empty: bool,
}

#[derive(Debug)]
pub struct Text<'a> {
    pub text: StrSpan<'a>,
}

pub trait ElementParser<'a, E> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<E, Error>;
}

pub trait AttributesParser<'a, A> {
    fn parse_attributes(&mut self) -> Result<A, Error>;
}

pub trait ChildrenParser<'a, C> {
    fn parse_children(&mut self) -> Result<C, Error>;
}

pub struct MrmlParser<'a> {
    tokenizer: Tokenizer<'a>,
    pub options: Arc<ParserOptions>,
    buffer: Vec<MrmlToken<'a>>,
}

impl<'a> MrmlParser<'a> {
    pub fn new(source: &'a str, options: Arc<ParserOptions>) -> Self {
        Self {
            tokenizer: Tokenizer::from(source),
            options,
            buffer: Default::default(),
        }
    }

    pub fn new_child<'b>(&self, source: &'b str) -> MrmlParser<'b> {
        MrmlParser {
            tokenizer: Tokenizer::from(source),
            options: self.options.clone(),
            buffer: Default::default(),
        }
    }

    fn read_next_token(&mut self) -> Option<Result<MrmlToken<'a>, Error>> {
        self.tokenizer
            .next()
            .map(|res| res.map_err(Error::from).and_then(MrmlToken::try_from))
            .and_then(|token| match token {
                Ok(MrmlToken::Text(inner)) if inner.text.trim().is_empty() => {
                    self.read_next_token()
                }
                other => Some(other),
            })
    }

    pub fn next_token(&mut self) -> Option<Result<MrmlToken<'a>, Error>> {
        if let Some(item) = self.buffer.pop() {
            Some(Ok(item))
        } else {
            self.read_next_token()
        }
    }

    pub fn rewind(&mut self, token: MrmlToken<'a>) {
        self.buffer.push(token);
    }

    pub fn assert_next(&mut self) -> Result<MrmlToken<'a>, Error> {
        self.next_token().unwrap_or_else(|| Err(Error::EndOfStream))
    }

    pub fn next_attribute(&mut self) -> Result<Option<Attribute<'a>>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::Attribute(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn next_element_end(&mut self) -> Result<Option<ElementEnd<'a>>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementEnd(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn assert_element_start(&mut self) -> Result<ElementStart<'a>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementStart(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::unexpected_token(other.range())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn assert_element_end(&mut self) -> Result<ElementEnd<'a>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementEnd(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::unexpected_token(other.range())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn assert_element_close(&mut self) -> Result<ElementClose<'a>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementClose(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::unexpected_token(other.range())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn next_text(&mut self) -> Result<Option<Text<'a>>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::Text(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn parse_root<T>(&mut self) -> Result<T, Error>
    where
        MrmlParser<'a>: ElementParser<'a, T>,
    {
        let start = self.assert_element_start()?;
        self.parse(start.local)
    }
}

impl<'a> AttributesParser<'a, Map<String, String>> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<Map<String, String>, Error> {
        let mut result = Map::new();
        while let Some(attr) = self.next_attribute()? {
            result.insert(attr.local.to_string(), attr.value.to_string());
        }
        Ok(result)
    }
}
