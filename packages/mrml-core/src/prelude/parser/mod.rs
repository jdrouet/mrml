use std::marker::PhantomData;

use htmlparser::{StrSpan, Tokenizer};

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

mod output;
mod token;

pub use output::*;
pub use token::*;

#[derive(Clone, Debug)]
pub enum Origin {
    Root,
    Include { path: String },
}

impl std::fmt::Display for Origin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root => write!(f, "root template"),
            Self::Include { path } => write!(f, "template from {path:?}"),
        }
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("unexpected element in {origin} at position {position}")]
    UnexpectedElement { origin: Origin, position: Span },
    #[error("unexpected token in {origin} at position {position}")]
    UnexpectedToken { origin: Origin, position: Span },
    #[error("missing attribute {name:?} in element in {origin} at position {position}")]
    MissingAttribute {
        name: &'static str,
        origin: Origin,
        position: Span,
    },
    #[error("invalid attribute in {origin} at position {position}")]
    InvalidAttribute { origin: Origin, position: Span },
    #[error("invalid format in {origin} at position {position}")]
    InvalidFormat { origin: Origin, position: Span },
    #[error("unexpected end of stream in {origin}")]
    EndOfStream { origin: Origin },
    /// The input string should be smaller than 4GiB.
    #[error("size limit reached in {origin}")]
    SizeLimit { origin: Origin },
    /// Errors detected by the `htmlparser` crate.
    #[error("unable to parse next template in {origin}")]
    ParserError {
        origin: Origin,
        #[source]
        source: htmlparser::Error,
    },
    /// The Mjml document must have at least one element.
    #[error("unable to find mjml element")]
    NoRootNode,
    #[error("unable to load included template in {origin} at position {position}")]
    IncludeLoaderError {
        origin: Origin,
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
    fn parse_attributes(&self, cursor: &mut MrmlCursor<'_>, tag: &StrSpan<'_>) -> Result<A, Error>;
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
    origin: Origin,
    warnings: Vec<Warning>,
}

impl<'a> MrmlCursor<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            tokenizer: Tokenizer::from(source),
            buffer: Default::default(),
            origin: Origin::Root,
            warnings: Default::default(),
        }
    }

    pub(crate) fn new_child<'b, O: Into<String>>(
        &self,
        origin: O,
        source: &'b str,
    ) -> MrmlCursor<'b> {
        MrmlCursor {
            tokenizer: Tokenizer::from(source),
            buffer: Default::default(),
            origin: Origin::Include {
                path: origin.into(),
            },
            warnings: Default::default(),
        }
    }

    pub(crate) fn origin(&self) -> Origin {
        self.origin.clone()
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
        tag: &StrSpan<'_>,
    ) -> Result<(A, C), Error>
    where
        MrmlParser<'opts>: ParseAttributes<A>,
        MrmlParser<'opts>: ParseChildren<C>,
        C: Default,
    {
        let attributes: A = self.parse_attributes(cursor, tag)?;
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok((attributes, Default::default()));
        }

        let children: C = self.parse_children(cursor)?;

        cursor.assert_element_close()?;

        Ok((attributes, children))
    }
}

impl ParseAttributes<Map<String, Option<String>>> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<Map<String, Option<String>>, Error> {
        parse_attributes_map(cursor)
    }
}

impl ParseAttributes<()> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<(), Error> {
        parse_attributes_empty(cursor)
    }
}

impl ParseChildren<String> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<String, Error> {
        Ok(cursor
            .next_text()?
            .map(|inner| inner.text.to_string())
            .unwrap_or_default())
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
        tag: &StrSpan<'_>,
    ) -> Result<(A, C), Error>
    where
        AsyncMrmlParser: ParseAttributes<A>,
        AsyncMrmlParser: AsyncParseChildren<C>,
        C: Default,
    {
        let attributes: A = self.parse_attributes(cursor, tag)?;
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
impl ParseAttributes<Map<String, Option<String>>> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<Map<String, Option<String>>, Error> {
        parse_attributes_map(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<()> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<(), Error> {
        parse_attributes_empty(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<String> for AsyncMrmlParser {
    async fn async_parse_children<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<String, Error> {
        Ok(cursor
            .next_text()?
            .map(|inner| inner.text.to_string())
            .unwrap_or_default())
    }
}

pub(crate) fn parse_attributes_map(
    cursor: &mut MrmlCursor<'_>,
) -> Result<Map<String, Option<String>>, Error> {
    let mut result = Map::new();
    while let Some(attr) = cursor.next_attribute()? {
        result.insert(
            attr.local.to_string(),
            attr.value.map(|inner| inner.to_string()),
        );
    }
    Ok(result)
}

pub(crate) fn parse_attributes_empty(cursor: &mut MrmlCursor<'_>) -> Result<(), Error> {
    if let Some(attr) = cursor.next_attribute()? {
        cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
    }
    Ok(())
}

impl<'opts, Tag: super::StaticTag, A, C> ParseElement<super::Component<PhantomData<Tag>, A, C>>
    for MrmlParser<'opts>
where
    MrmlParser<'opts>: ParseAttributes<A> + ParseChildren<C>,
    C: Default,
{
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<super::Component<PhantomData<Tag>, A, C>, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor, &tag)?;

        Ok(super::Component {
            tag: PhantomData::<Tag>,
            attributes,
            children,
        })
    }
}

impl<'opts, Tag: super::StaticTag, A> ParseElement<super::Component<PhantomData<Tag>, A, ()>>
    for MrmlParser<'opts>
where
    MrmlParser<'opts>: ParseAttributes<A>,
{
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<super::Component<PhantomData<Tag>, A, ()>, Error> {
        let attributes = self.parse_attributes(cursor, &tag)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            cursor.assert_element_close()?;
        }

        Ok(super::Component {
            tag: PhantomData::<Tag>,
            attributes,
            children: (),
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<Tag: super::StaticTag, A, C> AsyncParseElement<super::Component<PhantomData<Tag>, A, C>>
    for AsyncMrmlParser
where
    AsyncMrmlParser: ParseAttributes<A> + AsyncParseChildren<C>,
    A: std::marker::Send,
    C: Default,
{
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<super::Component<PhantomData<Tag>, A, C>, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor, &tag).await?;

        Ok(super::Component {
            tag: PhantomData::<Tag>,
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<Tag: super::StaticTag, A> AsyncParseElement<super::Component<PhantomData<Tag>, A, ()>>
    for AsyncMrmlParser
where
    AsyncMrmlParser: ParseAttributes<A>,
    A: std::marker::Send,
{
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<super::Component<PhantomData<Tag>, A, ()>, Error> {
        let attributes = self.parse_attributes(cursor, &tag)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            cursor.assert_element_close()?;
        }

        Ok(super::Component {
            tag: PhantomData::<Tag>,
            attributes,
            children: (),
        })
    }
}

#[cfg(test)]
#[macro_export]
macro_rules! should_parse {
    ($name: ident, $target: ty, $template: literal) => {
        $crate::should_sync_parse!($name, $target, $template);
        $crate::should_async_parse!($name, $target, $template);
    };
    ($name: ident, $target: ty, $template: literal, $warnings: literal) => {
        $crate::should_sync_parse!($name, $target, $template, $warnings);
        $crate::should_async_parse!($name, $target, $template, $warnings);
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_sync_parse {
    ($name: ident, $target: ty, $template: literal) => {
        $crate::should_sync_parse!($name, $target, $template, 0);
    };
    ($name: ident, $target: ty, $template: literal, $warnings: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, sync {
            #[test]
            fn fn_name() {
                let opts = $crate::prelude::parser::ParserOptions::default();
                let parser = $crate::prelude::parser::MrmlParser::new(&opts);
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).unwrap();
                assert_eq!(cursor.warnings().len(), $warnings);
            }
        });
    };
}

#[cfg(test)]
#[macro_export]
macro_rules! should_async_parse {
    ($name: ident, $target: ty, $template: literal) => {
        $crate::should_async_parse!($name, $target, $template, 0);
    };
    ($name: ident, $target: ty, $template: literal, $warnings: literal) => {
        concat_idents::concat_idents!(fn_name = $name, _, "async" {
            #[cfg(feature = "async")]
            #[tokio::test]
            async fn fn_name() {
                let parser = $crate::prelude::parser::AsyncMrmlParser::default();
                let mut cursor = $crate::prelude::parser::MrmlCursor::new($template);
                let _: $target = parser.parse_root(&mut cursor).await.unwrap();
                assert_eq!(cursor.warnings().len(), $warnings);
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
