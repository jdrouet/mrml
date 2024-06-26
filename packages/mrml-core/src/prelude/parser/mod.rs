use std::convert::TryFrom;
use std::fmt::Display;

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

#[cfg(feature = "async")]
#[derive(Debug)]
pub struct AsyncParserOptions {
    pub include_loader: Box<dyn loader::AsyncIncludeLoader + Send + Sync>,
}

#[cfg(feature = "async")]
#[allow(clippy::box_default)]
impl Default for AsyncParserOptions {
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

pub(crate) trait ParseElement<E> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, tag: StrSpan<'a>) -> Result<E, Error>;
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub(crate) trait AsyncParseElement<E> {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<E, Error>;
}

pub(crate) trait ParseAttributes<A> {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<A, Error>;
}

pub(crate) trait ParseChildren<C> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<C, Error>;
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
pub(crate) trait AsyncParseChildren<C> {
    async fn async_parse_children<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<C, Error>;
}

pub struct MrmlCursor<'a> {
    tokenizer: Tokenizer<'a>,
    buffer: Vec<MrmlToken<'a>>,
}

impl<'a> MrmlCursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::from(source),
            buffer: Default::default(),
        }
    }

    pub(crate) fn new_child<'b>(&self, source: &'b str) -> MrmlCursor<'b> {
        MrmlCursor {
            tokenizer: Tokenizer::from(source),
            buffer: Default::default(),
        }
    }

    fn read_next_token(&mut self) -> Option<Result<MrmlToken<'a>, Error>> {
        self.tokenizer
            .next()
            .map(|res| res.map_err(Error::from).and_then(MrmlToken::try_from))
            .and_then(|token| match token {
                Ok(MrmlToken::Text(inner))
                    if inner.text.starts_with('\n') && inner.text.trim().is_empty() =>
                {
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
            Some(Ok(MrmlToken::Text(inner))) if inner.text.trim().is_empty() => {
                self.assert_element_close()
            }
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
}

pub struct MrmlParser<'opts> {
    pub(crate) options: &'opts ParserOptions,
}

impl<'opts> MrmlParser<'opts> {
    pub fn new(options: &'opts ParserOptions) -> Self {
        Self { options }
    }
}

impl<'opts> MrmlParser<'opts> {
    pub(crate) fn parse_root<T>(&self, cursor: &mut MrmlCursor) -> Result<T, Error>
    where
        MrmlParser<'opts>: ParseElement<T>,
    {
        let start = cursor.assert_element_start()?;
        self.parse(cursor, start.local)
    }

    pub(crate) fn parse_attributes_and_children<A, C>(
        &self,
        cursor: &mut MrmlCursor,
    ) -> Result<(A, C), Error>
    where
        MrmlParser<'opts>: ParseAttributes<A>,
        MrmlParser<'opts>: ParseChildren<C>,
        C: Default,
    {
        let attributes: A = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok((attributes, Default::default()));
        }

        let children: C = self.parse_children(cursor)?;

        cursor.assert_element_close()?;

        Ok((attributes, children))
    }
}

impl<'opts> ParseAttributes<Map<String, String>> for MrmlParser<'opts> {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<Map<String, String>, Error> {
        parse_attributes_map(cursor)
    }
}

#[cfg(feature = "async")]
#[derive(Default)]
pub struct AsyncMrmlParser {
    pub(crate) options: std::sync::Arc<AsyncParserOptions>,
}

#[cfg(feature = "async")]
impl AsyncMrmlParser {
    pub fn new(options: std::sync::Arc<AsyncParserOptions>) -> Self {
        Self { options }
    }
}

#[cfg(feature = "async")]
impl AsyncMrmlParser {
    pub(crate) async fn parse_root<T>(&self, cursor: &mut MrmlCursor<'_>) -> Result<T, Error>
    where
        AsyncMrmlParser: AsyncParseElement<T>,
    {
        let start = cursor.assert_element_start()?;
        self.async_parse(cursor, start.local).await
    }

    pub(crate) async fn parse_attributes_and_children<A, C>(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<(A, C), Error>
    where
        AsyncMrmlParser: ParseAttributes<A>,
        AsyncMrmlParser: AsyncParseChildren<C>,
        C: Default,
    {
        let attributes: A = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok((attributes, Default::default()));
        }

        let children: C = self.async_parse_children(cursor).await?;

        cursor.assert_element_close()?;

        Ok((attributes, children))
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<Map<String, String>> for AsyncMrmlParser {
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>) -> Result<Map<String, String>, Error> {
        parse_attributes_map(cursor)
    }
}

pub(crate) fn parse_attributes_map(
    cursor: &mut MrmlCursor<'_>,
) -> Result<Map<String, String>, Error> {
    let mut result = Map::new();
    while let Some(attr) = cursor.next_attribute()? {
        result.insert(attr.local.to_string(), attr.value.to_string());
    }
    Ok(result)
}

#[cfg(test)]
#[macro_export]
macro_rules! should_parse {
    ($name: ident, $target: ty, $template: literal) => {
        $crate::should_sync_parse!($name, $target, $template);
        $crate::should_async_parse!($name, $target, $template);
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_sync_parse {
    ($name: ident, $target: ty, $template: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, sync {
            #[test]
            fn fn_name() {
                let opts = $crate::prelude::parser::ParserOptions::default();
                let parser = $crate::prelude::parser::MrmlParser::new(&opts);
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).unwrap();
            }
        });
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_async_parse {
    ($name: ident, $target: ty, $template: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, "async" {
            #[cfg(feature = "async")]
            #[tokio::test]
            async fn fn_name() {
                let parser = $crate::prelude::parser::AsyncMrmlParser::default();
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).await.unwrap();
            }
        });
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_not_parse {
    ($name: ident, $target: ty, $template: literal) => {
        $crate::should_not_sync_parse!($name, $target, $template);
        $crate::should_not_async_parse!($name, $target, $template);
    };
    ($name: ident, $target: ty, $template: literal, $message: literal) => {
        $crate::should_not_sync_parse!($name, $target, $template, $message);
        $crate::should_not_async_parse!($name, $target, $template, $message);
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_not_sync_parse {
    ($name: ident, $target: ty, $template: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, sync {
            #[test]
            #[should_panic]
            fn $name() {
                let opts = $crate::prelude::parser::ParserOptions::default();
                let parser = $crate::prelude::parser::MrmlParser::new(&opts);
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).unwrap();
            }
        });
    };
    ($name: ident, $target: ty, $template: literal, $message: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, sync {
            #[test]
            #[should_panic(expected = $message)]
            fn $name() {
                let opts = $crate::prelude::parser::ParserOptions::default();
                let parser = $crate::prelude::parser::MrmlParser::new(&opts);
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).unwrap();
            }
        });
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_not_async_parse {
    ($name: ident, $target: ty, $template: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, "async" {
            #[cfg(feature = "async")]
            #[tokio::test]
            #[should_panic]
            async fn fn_name() {
                let parser = $crate::prelude::parser::AsyncMrmlParser::default();
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).await.unwrap();
            }
        });
    };
    ($name: ident, $target: ty, $template: literal, $message: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, "async" {
            #[cfg(feature = "async")]
            #[tokio::test]
            #[should_panic(expected = $message)]
            async fn fn_name() {
                let parser = $crate::prelude::parser::AsyncMrmlParser::default();
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).await.unwrap();
            }
        });
    };
}
