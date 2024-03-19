use std::convert::TryFrom;

use xmlparser::StrSpan;

use super::{MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild, MjIncludeHeadKind};
use crate::comment::Comment;
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_style::NAME as MJ_STYLE;
use crate::mj_title::NAME as MJ_TITLE;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
};
use crate::text::Text;

impl<'opts> ParseElement<MjIncludeHeadChild> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self
                .parse(cursor, tag)
                .map(MjIncludeHeadChild::MjAttributes),
            MJ_BREAKPOINT => self
                .parse(cursor, tag)
                .map(MjIncludeHeadChild::MjBreakpoint),
            MJ_FONT => self.parse(cursor, tag).map(MjIncludeHeadChild::MjFont),
            MJ_PREVIEW => self.parse(cursor, tag).map(MjIncludeHeadChild::MjPreview),
            MJ_RAW => self.parse(cursor, tag).map(MjIncludeHeadChild::MjRaw),
            MJ_STYLE => self.parse(cursor, tag).map(MjIncludeHeadChild::MjStyle),
            MJ_TITLE => self.parse(cursor, tag).map(MjIncludeHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjIncludeHeadChild> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjAttributes),
            MJ_BREAKPOINT => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjBreakpoint),
            MJ_FONT => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjFont),
            MJ_PREVIEW => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjPreview),
            MJ_RAW => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjRaw),
            MJ_STYLE => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjStyle),
            MJ_TITLE => self
                .async_parse(cursor, tag)
                .await
                .map(MjIncludeHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

#[inline]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjIncludeHeadAttributes, Error> {
    let mut path = None;
    let mut kind = None;
    while let Some(attr) = cursor.next_attribute()? {
        match attr.local.as_str() {
            "path" => {
                path = Some(attr.value.to_string());
            }
            "type" => {
                kind = Some(MjIncludeHeadKind::try_from(attr.value)?);
            }
            _ => {
                return Err(Error::UnexpectedAttribute(attr.span.into()));
            }
        }
    }
    Ok(MjIncludeHeadAttributes {
        path: path.ok_or_else(|| Error::MissingAttribute("path", Default::default()))?,
        kind: kind.unwrap_or_default(),
    })
}

impl<'opts> ParseAttributes<MjIncludeHeadAttributes> for MrmlParser<'opts> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<MjIncludeHeadAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjIncludeHeadAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<MjIncludeHeadAttributes, Error> {
        parse_attributes(cursor)
    }
}

impl<'opts> ParseChildren<Vec<MjIncludeHeadChild>> for MrmlParser<'opts> {
    fn parse_children(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<Vec<MjIncludeHeadChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeHeadChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeHeadChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
        Ok(result)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjIncludeHeadChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjIncludeHeadChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeHeadChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeHeadChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.async_parse(cursor, inner.local).await?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
        Ok(result)
    }
}

impl<'opts> ParseElement<MjIncludeHead> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHead, Error> {
        let (attributes, children): (MjIncludeHeadAttributes, Vec<MjIncludeHeadChild>) =
            self.parse_attributes_and_children(cursor)?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeHeadChild> = if children.is_empty() {
            let child = self
                .options
                .include_loader
                .resolve(&attributes.path)
                .map_err(|source| Error::IncludeLoaderError {
                    position: tag.into(),
                    source,
                })?;

            match attributes.kind {
                MjIncludeHeadKind::Css { inline: false } => {
                    vec![MjIncludeHeadChild::MjStyle(crate::mj_style::MjStyle::from(
                        child,
                    ))]
                }
                MjIncludeHeadKind::Css { inline: true } => unimplemented!(),
                MjIncludeHeadKind::Mjml => {
                    let mut sub = cursor.new_child(child.as_str());
                    self.parse_children(&mut sub)?
                }
                MjIncludeHeadKind::Html => todo!(),
            }
        } else {
            children
        };

        Ok(MjIncludeHead {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjIncludeHead> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHead, Error> {
        let (attributes, children): (MjIncludeHeadAttributes, Vec<MjIncludeHeadChild>) =
            self.parse_attributes_and_children(cursor).await?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeHeadChild> = if children.is_empty() {
            let child = self
                .options
                .include_loader
                .async_resolve(&attributes.path)
                .await
                .map_err(|source| Error::IncludeLoaderError {
                    position: tag.into(),
                    source,
                })?;

            match attributes.kind {
                MjIncludeHeadKind::Css { inline: false } => {
                    vec![MjIncludeHeadChild::MjStyle(crate::mj_style::MjStyle::from(
                        child,
                    ))]
                }
                MjIncludeHeadKind::Css { inline: true } => unimplemented!(),
                MjIncludeHeadKind::Mjml => {
                    let mut sub = cursor.new_child(child.as_str());
                    self.async_parse_children(&mut sub).await?
                }
                MjIncludeHeadKind::Html => unimplemented!(),
            }
        } else {
            children
        };

        Ok(MjIncludeHead {
            attributes,
            children,
        })
    }
}

impl<'a> TryFrom<StrSpan<'a>> for MjIncludeHeadKind {
    type Error = Error;

    fn try_from(s: StrSpan<'a>) -> Result<Self, Self::Error> {
        match s.as_str() {
            "html" => Ok(Self::Html),
            "mjml" => Ok(Self::Mjml),
            "css" => Ok(Self::Css { inline: false }),
            _ => Err(Error::InvalidAttribute(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use xmlparser::StrSpan;

    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadKind};
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};

    #[test]
    fn should_parse_every_kind() {
        assert_eq!(
            MjIncludeHeadKind::try_from(StrSpan::from("html")).unwrap(),
            MjIncludeHeadKind::Html
        );
        assert_eq!(
            MjIncludeHeadKind::try_from(StrSpan::from("mjml")).unwrap(),
            MjIncludeHeadKind::Mjml
        );
        assert_eq!(
            MjIncludeHeadKind::try_from(StrSpan::from("css")).unwrap(),
            MjIncludeHeadKind::Css { inline: false }
        );
        assert!(MjIncludeHeadKind::try_from(StrSpan::from("other")).is_err());
    }

    crate::should_not_parse!(
        should_error_when_no_path,
        MjIncludeHead,
        "<mj-include />",
        "MissingAttribute(\"path\", Span { start: 0, end: 0 })"
    );

    crate::should_not_parse!(
        should_error_when_unknown_attribute,
        MjIncludeHead,
        r#"<mj-include unknown="yep" />"#,
        "UnexpectedAttribute(Span { start: 12, end: 25 })"
    );

    crate::should_parse!(
        should_parse_all_children,
        MjIncludeHead,
        r#"<mj-include path="inmemory">
    <mj-attributes />
    <mj-breakpoint width="400px" />
    <mj-font name="Comic" href="..." />
    <mj-preview>Preview</mj-preview>
    <mj-raw />
    <mj-style />
    <mj-title>Title</mj-title>
    <!-- Comment -->
</mj-include>"#
    );

    crate::should_not_parse!(
        should_error_unknown_children,
        MjIncludeHead,
        r#"<mj-include path="inmemory"><div /></mj-include>"#,
        "UnexpectedElement(Span { start: 29, end: 32 })"
    );

    crate::should_not_parse!(
        basic_in_noop_resolver,
        MjIncludeHead,
        r#"<mj-include path="basic.mjml" />"#,
        "IncludeLoaderError { position: Span { start: 1, end: 11 }, source: IncludeLoaderError { path: \"basic.mjml\", reason: NotFound, message: None, cause: None } }"
    );

    #[test]
    fn basic_in_memory_resolver_sync() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-title>Hello</mj-title>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(include.attributes.kind, MjIncludeHeadKind::Mjml);
        let _content = include.children.first().unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn basic_in_memory_resolver_async() {
        use crate::prelude::parser::{AsyncMrmlParser, AsyncParserOptions};

        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-title>Hello</mj-title>")]);
        let opts = AsyncParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let parser = AsyncMrmlParser::new(opts.into());
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeHead = parser.parse_root(&mut cursor).await.unwrap();
        assert_eq!(include.attributes.kind, MjIncludeHeadKind::Mjml);
        let _content = include.children.first().unwrap();
    }

    #[test]
    fn type_css_in_memory_resolver_sync() {
        let resolver =
            MemoryIncludeLoader::from(vec![("partial.css", "* { background-color: red; }")]);
        let raw = r#"<mj-include path="partial.css" type="css" />"#;
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(
            include.attributes.kind,
            MjIncludeHeadKind::Css { inline: false }
        );
        let _content = include.children.first().unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn type_css_in_memory_resolver_async() {
        use crate::prelude::parser::{AsyncMrmlParser, AsyncParserOptions};

        let resolver =
            MemoryIncludeLoader::from(vec![("partial.css", "* { background-color: red; }")]);
        let raw = r#"<mj-include path="partial.css" type="css" />"#;
        let opts = AsyncParserOptions {
            include_loader: Box::new(resolver),
        };
        let parser = AsyncMrmlParser::new(opts.into());
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeHead = parser.parse_root(&mut cursor).await.unwrap();
        assert_eq!(
            include.attributes.kind,
            MjIncludeHeadKind::Css { inline: false }
        );
        let _content = include.children.first().unwrap();
    }
}
