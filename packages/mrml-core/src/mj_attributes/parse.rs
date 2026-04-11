use htmlparser::StrSpan;

use super::MjAttributesChild;
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::mj_include::NAME as MJ_INCLUDE;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement, WarningKind,
};

const FRAGMENT_OPEN: &str = "<mrml-fragment>";
const FRAGMENT_CLOSE: &str = "</mrml-fragment>";

impl ParseElement<MjAttributesChild> for MrmlParser<'_> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesChild, Error> {
        Ok(match tag.as_str() {
            MJ_ALL => MjAttributesChild::MjAttributesAll(self.parse(cursor, tag)?),
            MJ_CLASS => MjAttributesChild::MjAttributesClass(self.parse(cursor, tag)?),
            _ => MjAttributesChild::MjAttributesElement(self.parse(cursor, tag)?),
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjAttributesChild> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesChild, Error> {
        Ok(match tag.as_str() {
            MJ_ALL => MjAttributesChild::MjAttributesAll(self.async_parse(cursor, tag).await?),
            MJ_CLASS => MjAttributesChild::MjAttributesClass(self.async_parse(cursor, tag).await?),
            _ => MjAttributesChild::MjAttributesElement(self.async_parse(cursor, tag).await?),
        })
    }
}

impl MrmlParser<'_> {
    fn parse_mj_include_attributes_children(
        &self,
        cursor: &mut MrmlCursor<'_>,
        tag: StrSpan<'_>,
    ) -> Result<Vec<MjAttributesChild>, Error> {
        let mut path = None;
        while let Some(attr) = cursor.next_attribute()? {
            if attr.local.as_str() == "path" {
                path = attr.value.map(|v| v.to_string());
            } else {
                cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
            }
        }
        let path = path.ok_or_else(|| Error::MissingAttribute {
            name: "path",
            origin: cursor.origin(),
            position: tag.into(),
        })?;

        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            let children: Vec<MjAttributesChild> = self.parse_children(cursor)?;
            cursor.assert_element_close()?;
            return Ok(children);
        }

        let content = self
            .options
            .include_loader
            .resolve(&path)
            .map_err(|source| Error::IncludeLoaderError {
                origin: cursor.origin(),
                position: tag.into(),
                source,
            })?;

        let wrapped = format!("{FRAGMENT_OPEN}{content}{FRAGMENT_CLOSE}");
        let offset = FRAGMENT_OPEN.len();
        let with_position = |err: Error| err.adjust_positions(offset);
        let mut sub = cursor.new_child(&path, wrapped.as_str());
        sub.set_source_offset(offset);
        sub.assert_element_start().map_err(&with_position)?;
        sub.assert_element_end().map_err(&with_position)?;
        let children: Vec<MjAttributesChild> =
            self.parse_children(&mut sub).map_err(&with_position)?;
        sub.assert_element_close().map_err(&with_position)?;
        cursor.with_warnings(sub.warnings());
        Ok(children)
    }
}

impl ParseChildren<Vec<MjAttributesChild>> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjAttributesChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_INCLUDE {
                        result.extend(
                            self.parse_mj_include_attributes_children(cursor, inner.local)?,
                        );
                    } else {
                        result.push(self.parse(cursor, inner.local)?);
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }
    }
}

#[cfg(feature = "async")]
impl AsyncMrmlParser {
    async fn async_parse_mj_include_attributes_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<Vec<MjAttributesChild>, Error> {
        let mut path = None;
        while let Some(attr) = cursor.next_attribute()? {
            if attr.local.as_str() == "path" {
                path = attr.value.map(|v| v.to_string());
            } else {
                cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
            }
        }
        let path = path.ok_or_else(|| Error::MissingAttribute {
            name: "path",
            origin: cursor.origin(),
            position: tag.into(),
        })?;

        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            let children: Vec<MjAttributesChild> = self.async_parse_children(cursor).await?;
            cursor.assert_element_close()?;
            return Ok(children);
        }

        let content = self
            .options
            .include_loader
            .async_resolve(&path)
            .await
            .map_err(|source| Error::IncludeLoaderError {
                origin: cursor.origin(),
                position: tag.into(),
                source,
            })?;

        let wrapped = format!("{FRAGMENT_OPEN}{content}{FRAGMENT_CLOSE}");
        let offset = FRAGMENT_OPEN.len();
        let with_position = |err: Error| err.adjust_positions(offset);
        let mut sub = cursor.new_child(&path, wrapped.as_str());
        sub.set_source_offset(offset);
        sub.assert_element_start().map_err(&with_position)?;
        sub.assert_element_end().map_err(&with_position)?;
        let children: Vec<MjAttributesChild> = self
            .async_parse_children(&mut sub)
            .await
            .map_err(&with_position)?;
        sub.assert_element_close().map_err(&with_position)?;
        cursor.with_warnings(sub.warnings());
        Ok(children)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjAttributesChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjAttributesChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_INCLUDE {
                        result.extend(
                            self.async_parse_mj_include_attributes_children(cursor, inner.local)
                                .await?,
                        );
                    } else {
                        result.push(self.async_parse(cursor, inner.local).await?);
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes::MjAttributes;

    crate::should_sync_parse!(
        parse_complete,
        MjAttributes,
        r#"
    <mj-attributes>
        <mj-all color="red" />
        <mj-class name="head" color="green" />
        <span color="blue" />
    </mj-attributes>
"#
    );

    #[test]
    fn should_parse_mj_include_in_attributes_sync() {
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};

        let resolver = MemoryIncludeLoader::from(vec![(
            "partial.mjml",
            r#"<mj-button background-color="red" />"#,
        )]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-attributes><mj-include path="partial.mjml" /></mj-attributes>"#;
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(raw);
        let result: MjAttributes = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(result.children.len(), 1);
        assert!(result.children[0].as_mj_attributes_element().is_some());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn should_parse_mj_include_in_attributes_async() {
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::{AsyncMrmlParser, AsyncParserOptions, MrmlCursor};

        let resolver = MemoryIncludeLoader::from(vec![(
            "partial.mjml",
            r#"<mj-button background-color="red" />"#,
        )]);
        let opts = AsyncParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-attributes><mj-include path="partial.mjml" /></mj-attributes>"#;
        let parser = AsyncMrmlParser::new(opts.into());
        let mut cursor = MrmlCursor::new(raw);
        let result: MjAttributes = parser.parse_root(&mut cursor).await.unwrap();
        assert_eq!(result.children.len(), 1);
        assert!(result.children[0].as_mj_attributes_element().is_some());
    }

    #[test]
    fn should_parse_open_close_include_with_no_children() {
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};

        let resolver = MemoryIncludeLoader::from(vec![(
            "partial.mjml",
            r#"<mj-button background-color="red" />"#,
        )]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        // open/close form with no inline children should NOT load from file
        let raw = r#"<mj-attributes><mj-include path="partial.mjml"></mj-include></mj-attributes>"#;
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(raw);
        let result: MjAttributes = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(result.children.len(), 0);
    }

    #[test]
    fn should_warn_on_unexpected_attribute() {
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions, WarningKind};

        let resolver = MemoryIncludeLoader::from(vec![(
            "partial.mjml",
            r#"<mj-button background-color="red" />"#,
        )]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-attributes><mj-include foo="bar" path="partial.mjml" /></mj-attributes>"#;
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(raw);
        let _: MjAttributes = parser.parse_root(&mut cursor).unwrap();
        let warnings = cursor.warnings();
        assert_eq!(warnings.len(), 1);
        assert_eq!(warnings[0].kind, WarningKind::UnexpectedAttribute);
    }

    #[test]
    #[should_panic(expected = "MissingAttribute")]
    fn should_error_on_missing_path() {
        let opts = crate::prelude::parser::ParserOptions::default();
        let raw = r#"<mj-attributes><mj-include /></mj-attributes>"#;
        let parser = crate::prelude::parser::MrmlParser::new(&opts);
        let mut cursor = crate::prelude::parser::MrmlCursor::new(raw);
        let _: MjAttributes = parser.parse_root(&mut cursor).unwrap();
    }
}
