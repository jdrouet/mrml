use htmlparser::StrSpan;

use super::{MjIncludeBody, MjIncludeBodyAttributes, MjIncludeBodyChild, MjIncludeBodyKind};
use crate::comment::Comment;
use crate::mj_accordion::NAME as MJ_ACCORDION;
use crate::mj_body::MjBodyChild;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_carousel::NAME as MJ_CAROUSEL;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_group::NAME as MJ_GROUP;
use crate::mj_hero::NAME as MJ_HERO;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_navbar::NAME as MJ_NAVBAR;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_social::NAME as MJ_SOCIAL;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_table::NAME as MJ_TABLE;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::{MjWrapper, NAME as MJ_WRAPPER};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
    WarningKind,
};
use crate::text::Text;

impl ParseElement<MjIncludeBodyChild> for MrmlParser<'_> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeBodyChild, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjIncludeBodyChild::MjAccordion(self.parse(cursor, tag)?)),
            MJ_BUTTON => Ok(MjIncludeBodyChild::MjButton(self.parse(cursor, tag)?)),
            MJ_CAROUSEL => Ok(MjIncludeBodyChild::MjCarousel(self.parse(cursor, tag)?)),
            MJ_COLUMN => Ok(MjIncludeBodyChild::MjColumn(self.parse(cursor, tag)?)),
            MJ_DIVIDER => Ok(MjIncludeBodyChild::MjDivider(self.parse(cursor, tag)?)),
            MJ_GROUP => Ok(MjIncludeBodyChild::MjGroup(self.parse(cursor, tag)?)),
            MJ_HERO => Ok(MjIncludeBodyChild::MjHero(self.parse(cursor, tag)?)),
            MJ_IMAGE => Ok(MjIncludeBodyChild::MjImage(self.parse(cursor, tag)?)),
            MJ_NAVBAR => Ok(MjIncludeBodyChild::MjNavbar(self.parse(cursor, tag)?)),
            MJ_RAW => Ok(MjIncludeBodyChild::MjRaw(self.parse(cursor, tag)?)),
            MJ_SECTION => Ok(MjIncludeBodyChild::MjSection(self.parse(cursor, tag)?)),
            MJ_SOCIAL => Ok(MjIncludeBodyChild::MjSocial(self.parse(cursor, tag)?)),
            MJ_SPACER => Ok(MjIncludeBodyChild::MjSpacer(self.parse(cursor, tag)?)),
            MJ_TABLE => Ok(MjIncludeBodyChild::MjTable(self.parse(cursor, tag)?)),
            MJ_TEXT => Ok(MjIncludeBodyChild::MjText(self.parse(cursor, tag)?)),
            MJ_WRAPPER => Ok(MjIncludeBodyChild::MjWrapper(self.parse(cursor, tag)?)),
            _ => Err(Error::UnexpectedElement {
                origin: cursor.origin(),
                position: tag.into(),
            }),
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjIncludeBodyChild> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeBodyChild, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjIncludeBodyChild::MjAccordion(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_BUTTON => Ok(MjIncludeBodyChild::MjButton(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_CAROUSEL => Ok(MjIncludeBodyChild::MjCarousel(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_COLUMN => Ok(MjIncludeBodyChild::MjColumn(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_DIVIDER => Ok(MjIncludeBodyChild::MjDivider(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_GROUP => Ok(MjIncludeBodyChild::MjGroup(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_HERO => Ok(MjIncludeBodyChild::MjHero(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_IMAGE => Ok(MjIncludeBodyChild::MjImage(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_NAVBAR => Ok(MjIncludeBodyChild::MjNavbar(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_RAW => Ok(MjIncludeBodyChild::MjRaw(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_SECTION => Ok(MjIncludeBodyChild::MjSection(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_SOCIAL => Ok(MjIncludeBodyChild::MjSocial(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_SPACER => Ok(MjIncludeBodyChild::MjSpacer(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_TABLE => Ok(MjIncludeBodyChild::MjTable(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_TEXT => Ok(MjIncludeBodyChild::MjText(
                self.async_parse(cursor, tag).await?,
            )),
            MJ_WRAPPER => Ok(MjIncludeBodyChild::MjWrapper(
                self.async_parse(cursor, tag).await?,
            )),
            _ => Err(Error::UnexpectedElement {
                origin: cursor.origin(),
                position: tag.into(),
            }),
        }
    }
}

impl MjIncludeBodyKind {
    fn maybe_parse(span: &StrSpan<'_>) -> Option<Self> {
        match span.as_str() {
            "html" => Some(Self::Html),
            "mjml" => Some(Self::Mjml),
            _ => None,
        }
    }

    fn parse(cursor: &mut MrmlCursor<'_>, span: StrSpan<'_>) -> Result<Self, Error> {
        Self::maybe_parse(&span).ok_or_else(|| Error::InvalidAttribute {
            origin: cursor.origin(),
            position: span.into(),
        })
    }
}

#[inline]
fn parse_attributes(
    cursor: &mut MrmlCursor<'_>,
    tag: &StrSpan<'_>,
) -> Result<MjIncludeBodyAttributes, Error> {
    let mut path = None;
    let mut kind: Option<MjIncludeBodyKind> = None;
    while let Some(attr) = cursor.next_attribute()? {
        match (attr.local.as_str(), attr.value) {
            ("path", Some(value)) => {
                path = Some(value.to_string());
            }
            ("type", Some(value)) => {
                kind = Some(MjIncludeBodyKind::parse(cursor, value)?);
            }
            _ => {
                cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
            }
        }
    }
    Ok(MjIncludeBodyAttributes {
        path: path.ok_or_else(|| Error::MissingAttribute {
            name: "path",
            origin: cursor.origin(),
            position: tag.into(),
        })?,
        kind: kind.unwrap_or_default(),
    })
}

impl ParseAttributes<MjIncludeBodyAttributes> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        tag: &StrSpan<'_>,
    ) -> Result<MjIncludeBodyAttributes, Error> {
        parse_attributes(cursor, tag)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjIncludeBodyAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        tag: &StrSpan<'_>,
    ) -> Result<MjIncludeBodyAttributes, Error> {
        parse_attributes(cursor, tag)
    }
}

impl ParseChildren<Vec<MjIncludeBodyChild>> for MrmlParser<'_> {
    fn parse_children(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<Vec<MjIncludeBodyChild>, Error> {
        let mut result = Vec::new();

        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeBodyChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeBodyChild::Text(Text::from(inner.text.as_str())));
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }

        Ok(result)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjIncludeBodyChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjIncludeBodyChild>, Error> {
        let mut result = Vec::new();

        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeBodyChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.async_parse(cursor, inner.local).await?);
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeBodyChild::Text(Text::from(inner.text.as_str())));
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }

        Ok(result)
    }
}

impl ParseElement<MjIncludeBody> for MrmlParser<'_> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeBody, Error> {
        let (attributes, children): (MjIncludeBodyAttributes, Vec<MjIncludeBodyChild>) =
            self.parse_attributes_and_children(cursor, &tag)?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeBodyChild> = if children.is_empty() {
            let child = self
                .options
                .include_loader
                .resolve(&attributes.path)
                .map_err(|source| Error::IncludeLoaderError {
                    origin: cursor.origin(),
                    position: tag.into(),
                    source,
                })?;
            match attributes.kind {
                MjIncludeBodyKind::Html => {
                    let mut sub = cursor.new_child(&attributes.path, child.as_str());
                    let children: Vec<MjBodyChild> = self.parse_children(&mut sub)?;
                    cursor.with_warnings(sub.warnings());
                    vec![MjIncludeBodyChild::MjWrapper(MjWrapper::new(
                        Default::default(),
                        children,
                    ))]
                }
                MjIncludeBodyKind::Mjml => {
                    let mut sub = cursor.new_child(&attributes.path, child.as_str());
                    let children = self.parse_children(&mut sub)?;
                    cursor.with_warnings(sub.warnings());
                    children
                }
            }
        } else {
            children
        };

        Ok(MjIncludeBody::new(attributes, children))
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl crate::prelude::parser::AsyncParseElement<MjIncludeBody> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeBody, Error> {
        use crate::prelude::parser::AsyncParseChildren;

        let (attributes, children): (MjIncludeBodyAttributes, Vec<MjIncludeBodyChild>) =
            self.parse_attributes_and_children(cursor, &tag).await?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeBodyChild> = if children.is_empty() {
            let child = self
                .options
                .include_loader
                .async_resolve(&attributes.path)
                .await
                .map_err(|source| Error::IncludeLoaderError {
                    origin: cursor.origin(),
                    position: tag.into(),
                    source,
                })?;
            match attributes.kind {
                MjIncludeBodyKind::Html => {
                    let mut sub = cursor.new_child(&attributes.path, child.as_str());
                    let children: Vec<MjBodyChild> = self.async_parse_children(&mut sub).await?;
                    vec![MjIncludeBodyChild::MjWrapper(MjWrapper::new(
                        Default::default(),
                        children,
                    ))]
                }
                MjIncludeBodyKind::Mjml => {
                    let mut sub = cursor.new_child(&attributes.path, child.as_str());
                    let children = self.async_parse_children(&mut sub).await?;
                    cursor.with_warnings(sub.warnings());
                    children
                }
            }
        } else {
            children
        };

        Ok(MjIncludeBody::new(attributes, children))
    }
}

#[cfg(test)]
mod tests {
    use htmlparser::StrSpan;

    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyKind};
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions, WarningKind};

    #[test]
    fn kind_parser() {
        assert_eq!(
            MjIncludeBodyKind::maybe_parse(&StrSpan::from("html")).unwrap(),
            MjIncludeBodyKind::Html
        );
        assert_eq!(
            MjIncludeBodyKind::maybe_parse(&StrSpan::from("mjml")).unwrap(),
            MjIncludeBodyKind::Mjml
        );
        assert!(MjIncludeBodyKind::maybe_parse(&StrSpan::from("foo")).is_none());
    }

    crate::should_not_parse!(
        invalid_kind,
        MjIncludeBody,
        r#"<mj-include type="foo" path="basic.mjml" />"#,
        "InvalidAttribute { origin: Root, position: Span { start: 18, end: 21 } }"
    );

    crate::should_not_parse!(
        not_found,
        MjIncludeBody,
        r#"<mj-include path="basic.mjml" />"#,
        "IncludeLoaderError { origin: Root, position: Span { start: 1, end: 11 }, source: IncludeLoaderError { path: \"basic.mjml\", reason: NotFound, message: None, cause: None } }"
    );

    crate::should_parse!(
        basic_with_children,
        MjIncludeBody,
        r#"<mj-include path="basic.mjml"><mj-text>Hello World</mj-text> <!-- Coucou --></mj-include>"#
    );

    #[test]
    fn basic_in_memory_resolver_sync() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeBody = MrmlParser::new(&opts).parse_root(&mut cursor).unwrap();
        assert_eq!(include.0.attributes.kind, MjIncludeBodyKind::Mjml);
        let _content = include.0.children.first().unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn basic_in_memory_resolver_async() {
        use crate::prelude::parser::{AsyncMrmlParser, AsyncParserOptions};

        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")]);
        let opts = AsyncParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeBody = AsyncMrmlParser::new(opts.into())
            .parse_root(&mut cursor)
            .await
            .unwrap();
        assert_eq!(include.0.attributes.kind, MjIncludeBodyKind::Mjml);
        let _content = include.0.children.first().unwrap();
    }

    #[test]
    fn type_html_in_memory_resolver_sync() {
        let resolver = MemoryIncludeLoader::from(vec![("partial.html", "<h1>Hello World!</h1>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="partial.html" type="html" />"#;
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeBody = MrmlParser::new(&opts).parse_root(&mut cursor).unwrap();
        assert_eq!(include.0.attributes.kind, MjIncludeBodyKind::Html);
        let _content = include.0.children.first().unwrap();
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn type_html_in_memory_resolver_async() {
        use crate::prelude::parser::{AsyncMrmlParser, AsyncParserOptions};

        let resolver = MemoryIncludeLoader::from(vec![("partial.html", "<h1>Hello World!</h1>")]);
        let opts = AsyncParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="partial.html" type="html" />"#;
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeBody = AsyncMrmlParser::new(opts.into())
            .parse_root(&mut cursor)
            .await
            .unwrap();
        assert_eq!(include.0.attributes.kind, MjIncludeBodyKind::Html);
        let _content = include.0.children.first().unwrap();
    }

    crate::should_parse!(
        parse_all_kind_of_children,
        MjIncludeBody,
        r#"<mj-include path="partial.html">
    <mj-accordion />
    <mj-button />
    <mj-carousel />
    <mj-column />
    <mj-divider />
    <mj-group />
    <mj-hero />
    <mj-image path="./here.png" />
    <mj-navbar />
    <mj-raw />
    <mj-section />
    <mj-social />
    <mj-spacer />
    <mj-table />
    <mj-text />
    <mj-wrapper />
    <!-- hello -->
    World
</mj-include>"#
    );

    crate::should_not_parse!(
        parse_unexpected_child,
        MjIncludeBody,
        r#"<mj-include path="partial.html">
    <foo />
</mj-include>"#,
        "UnexpectedElement { origin: Root, position: Span { start: 38, end: 41 } }"
    );

    crate::should_parse!(
        invalid_attribute,
        MjIncludeBody,
        r#"<mj-include invalid="attribute" path="partial.html"><!-- empty --></mj-include>"#,
        1
    );

    crate::should_not_parse!(
        missing_path,
        MjIncludeBody,
        r#"<mj-include><!-- empty --></mj-include>"#,
        "MissingAttribute { name: \"path\", origin: Root, position: Span { start: 1, end: 11 } }"
    );

    #[test]
    fn warnings_from_include_child() {
        let resolver = MemoryIncludeLoader::from(vec![(
            "partial.html",
            "<mj-raw foo=\"bar\"><h1>Hello World!</h1></mj-raw>",
        )]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="partial.html" type="html" />"#;
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeBody = MrmlParser::new(&opts).parse_root(&mut cursor).unwrap();
        assert_eq!(include.0.attributes.kind, MjIncludeBodyKind::Html);
        let warnings = cursor.warnings();
        assert_eq!(warnings.len(), 1);
        let warning = warnings.first().unwrap();
        assert_eq!(warning.kind, WarningKind::UnexpectedAttribute);
        assert_eq!(
            warning.to_string(),
            "unexpected attribute in template from \"partial.html\" at position 8:17"
        );
    }
}
