use std::{collections::HashMap, convert::TryFrom, rc::Rc};

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
    pub fn invalid_format((start, end): (usize, usize)) -> Self {
        Error::InvalidFormat(start, end)
    }

    pub fn unexpected_token((start, end): (usize, usize)) -> Self {
        Error::UnexpectedToken(start, end)
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

pub trait MaybeFromToken<'a>: Sized {
    fn maybe_from(token: Token<'a>) -> Result<Self, Token<'a>>;
}

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
            Token::Text { text } => Ok(MrmlToken::Text(Text { text })),
            other => Err(Error::unexpected_token(get_span(&other))),
        }
    }
}

impl<'a> TryFrom<MrmlToken<'a>> for Attribute<'a> {
    type Error = MrmlToken<'a>;

    fn try_from(value: MrmlToken<'a>) -> Result<Self, Self::Error> {
        match value {
            MrmlToken::Attribute(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl<'a> From<Attribute<'a>> for MrmlToken<'a> {
    fn from(value: Attribute<'a>) -> Self {
        Self::Attribute(value)
    }
}

impl<'a> TryFrom<MrmlToken<'a>> for Comment<'a> {
    type Error = MrmlToken<'a>;

    fn try_from(value: MrmlToken<'a>) -> Result<Self, Self::Error> {
        match value {
            MrmlToken::Comment(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl<'a> From<Comment<'a>> for MrmlToken<'a> {
    fn from(value: Comment<'a>) -> Self {
        Self::Comment(value)
    }
}

impl<'a> TryFrom<MrmlToken<'a>> for ElementClose<'a> {
    type Error = MrmlToken<'a>;

    fn try_from(value: MrmlToken<'a>) -> Result<Self, Self::Error> {
        match value {
            MrmlToken::ElementClose(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl<'a> From<ElementEnd<'a>> for MrmlToken<'a> {
    fn from(value: ElementEnd<'a>) -> Self {
        Self::ElementEnd(value)
    }
}

impl<'a> TryFrom<MrmlToken<'a>> for ElementEnd<'a> {
    type Error = MrmlToken<'a>;

    fn try_from(value: MrmlToken<'a>) -> Result<Self, Self::Error> {
        match value {
            MrmlToken::ElementEnd(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl<'a> From<ElementStart<'a>> for MrmlToken<'a> {
    fn from(value: ElementStart<'a>) -> Self {
        Self::ElementStart(value)
    }
}

impl<'a> TryFrom<MrmlToken<'a>> for ElementStart<'a> {
    type Error = MrmlToken<'a>;

    fn try_from(value: MrmlToken<'a>) -> Result<Self, Self::Error> {
        match value {
            MrmlToken::ElementStart(inner) => Ok(inner),
            other => Err(other),
        }
    }
}

impl<'a> From<Text<'a>> for MrmlToken<'a> {
    fn from(value: Text<'a>) -> Self {
        Self::Text(value)
    }
}

impl<'a> TryFrom<MrmlToken<'a>> for Text<'a> {
    type Error = MrmlToken<'a>;

    fn try_from(value: MrmlToken<'a>) -> Result<Self, Self::Error> {
        match value {
            MrmlToken::Text(inner) => Ok(inner),
            other => Err(other),
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

pub struct Attribute<'a> {
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub value: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

pub struct Comment<'a> {
    pub span: StrSpan<'a>,
    pub text: StrSpan<'a>,
}

pub struct ElementClose<'a> {
    pub span: StrSpan<'a>,
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
}

pub struct ElementStart<'a> {
    pub prefix: StrSpan<'a>,
    pub local: StrSpan<'a>,
    pub span: StrSpan<'a>,
}

pub struct ElementEnd<'a> {
    pub span: StrSpan<'a>,
    pub empty: bool,
}

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
    options: ParserOptions,
    buffer: Vec<MrmlToken<'a>>,
}

impl<'a> MrmlParser<'a> {
    fn read_next_token(&mut self) -> Option<Result<MrmlToken<'a>, Error>> {
        self.tokenizer
            .next()
            .map(|res| res.map_err(Error::from).and_then(MrmlToken::try_from))
    }

    pub fn next(&mut self) -> Option<Result<MrmlToken<'a>, Error>> {
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
        self.next().unwrap_or_else(|| Err(Error::EndOfStream))
    }

    pub fn next_attribute(&mut self) -> Result<Option<Attribute<'a>>, Error> {
        match self.next() {
            Some(Ok(MrmlToken::Attribute(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn next_element_start(&mut self) -> Result<Option<ElementStart<'a>>, Error> {
        match self.next() {
            Some(Ok(MrmlToken::ElementStart(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn next_element_end(&mut self) -> Result<Option<ElementEnd<'a>>, Error> {
        match self.next() {
            Some(Ok(MrmlToken::ElementEnd(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn assert_element_end(&mut self) -> Result<ElementEnd<'a>, Error> {
        match self.next() {
            Some(Ok(MrmlToken::ElementEnd(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::unexpected_token(other.range())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn assert_element_close(&mut self) -> Result<ElementClose<'a>, Error> {
        match self.next() {
            Some(Ok(MrmlToken::ElementClose(inner))) => Ok(inner),
            Some(Ok(other)) => Err(Error::unexpected_token(other.range())),
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }

    pub fn next_text(&mut self) -> Result<Option<Text<'a>>, Error> {
        match self.next() {
            Some(Ok(MrmlToken::Text(inner))) => Ok(Some(inner)),
            Some(Ok(other)) => {
                self.rewind(other);
                Ok(None)
            }
            Some(Err(inner)) => Err(inner),
            None => Err(Error::EndOfStream),
        }
    }
}

impl<'a> AttributesParser<'a, HashMap<String, String>> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<HashMap<String, String>, Error> {
        let mut result = HashMap::new();
        while let Some(attr) = self.next_attribute()? {
            result.insert(attr.local.to_string(), attr.value.to_string());
        }
        Ok(result)
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
