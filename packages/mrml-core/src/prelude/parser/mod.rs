use std::convert::TryFrom;
use std::fmt::Display;
use std::sync::Arc;

use xmlparser::{StrSpan, Token, Tokenizer};

use self::loader::IncludeLoaderError;
use super::hash::Map;

#[cfg(feature = "http-loader-base")]
pub mod http_loader;
pub mod loader;
#[cfg(feature = "local-loader")]
pub mod local_loader;
pub mod memory_loader;
pub mod multi_loader;
pub mod noop_loader;

#[derive(Clone, Debug, Default)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

impl Display for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}..{}", self.start, self.end)
    }
}

impl<'a> From<StrSpan<'a>> for Span {
    fn from(value: StrSpan<'a>) -> Self {
        Self {
            start: value.start(),
            end: value.end(),
        }
    }
}

impl<'a> From<Token<'a>> for Span {
    fn from(value: Token<'a>) -> Self {
        match value {
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
            | Token::ProcessingInstruction { span, .. } => span.into(),
            Token::Text { text } => text.into(),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected attribute at position {0}")]
    UnexpectedAttribute(Span),
    #[error("unexpected element at position {0}")]
    UnexpectedElement(Span),
    #[error("unexpected token at position {0}")]
    UnexpectedToken(Span),
    #[error("missing attribute {0} in element at position {1}")]
    MissingAttribute(&'static str, Span),
    #[error("invalid attribute at position {0}")]
    InvalidAttribute(Span),
    #[error("invalid format at position {0}")]
    InvalidFormat(Span),
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
    IncludeLoaderError {
        position: Span,
        #[source]
        source: IncludeLoaderError,
    },
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
pub(crate) enum MrmlToken<'a> {
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
            other => Err(Error::UnexpectedToken(other.into())),
        }
    }
}

impl<'a> MrmlToken<'a> {
    pub fn span(&self) -> Span {
        match self {
            Self::Attribute(item) => item.span,
            Self::Comment(item) => item.span,
            Self::ElementClose(item) => item.span,
            Self::ElementEnd(item) => item.span,
            Self::ElementStart(item) => item.span,
            Self::Text(item) => item.text,
        }
        .into()
    }
}

#[derive(Debug)]
pub(crate) struct Attribute<'a> {
    #[allow(unused)]
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub value: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct Comment<'a> {
    pub span: StrSpan<'a>,
    pub text: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct ElementClose<'a> {
    #[allow(unused)]
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct ElementStart<'a> {
    #[allow(unused)]
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

#[derive(Debug)]
pub(crate) struct ElementEnd<'a> {
    pub span: StrSpan<'a>,
    pub empty: bool,
}

#[derive(Debug)]
pub(crate) struct Text<'a> {
    pub text: StrSpan<'a>,
}

pub(crate) trait ElementParser<'a, E> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<E, Error>;
}

pub(crate) trait AttributesParser<'a, A> {
    fn parse_attributes(&mut self) -> Result<A, Error>;
}

pub(crate) trait ChildrenParser<'a, C> {
    fn parse_children(&mut self) -> Result<C, Error>;
}

pub struct MrmlParser<'a> {
    tokenizer: Tokenizer<'a>,
    pub(crate) options: Arc<ParserOptions>,
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

    pub(crate) fn new_child<'b>(&self, source: &'b str) -> MrmlParser<'b> {
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

    pub(crate) fn next_token(&mut self) -> Option<Result<MrmlToken<'a>, Error>> {
        if let Some(item) = self.buffer.pop() {
            Some(Ok(item))
        } else {
            self.read_next_token()
        }
    }

    pub(crate) fn rewind(&mut self, token: MrmlToken<'a>) {
        self.buffer.push(token);
    }

    pub(crate) fn assert_next(&mut self) -> Result<MrmlToken<'a>, Error> {
        self.next_token().unwrap_or_else(|| Err(Error::EndOfStream))
    }

    pub(crate) fn next_attribute(&mut self) -> Result<Option<Attribute<'a>>, Error> {
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

    pub(crate) fn assert_element_start(&mut self) -> Result<ElementStart<'a>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementStart(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::UnexpectedToken(other.span())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub(crate) fn assert_element_end(&mut self) -> Result<ElementEnd<'a>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementEnd(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::UnexpectedToken(other.span())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub(crate) fn assert_element_close(&mut self) -> Result<ElementClose<'a>, Error> {
        match self.next_token() {
            Some(Ok(MrmlToken::ElementClose(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::UnexpectedToken(other.span())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub(crate) fn next_text(&mut self) -> Result<Option<Text<'a>>, Error> {
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

    pub(crate) fn parse_root<T>(&mut self) -> Result<T, Error>
    where
        MrmlParser<'a>: ElementParser<'a, T>,
    {
        let start = self.assert_element_start()?;
        self.parse(start.local)
    }

    pub(crate) fn parse_attributes_and_children<A, C>(&mut self) -> Result<(A, C), Error>
    where
        MrmlParser<'a>: AttributesParser<'a, A>,
        MrmlParser<'a>: ChildrenParser<'a, C>,
        C: Default,
    {
        let attributes: A = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok((attributes, Default::default()));
        }

        let children: C = self.parse_children()?;

        self.assert_element_close()?;

        Ok((attributes, children))
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
