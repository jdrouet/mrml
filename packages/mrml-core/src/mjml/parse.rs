use htmlparser::StrSpan;

use super::{Mjml, MjmlAttributes, MjmlChildren};
use crate::mj_body::NAME as MJ_BODY;
use crate::mj_head::NAME as MJ_HEAD;
use crate::mj_include::NAME as MJ_INCLUDE;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren, ParseElement,
    ParseOutput, ParserOptions, WarningKind,
};

const WRAPPER_OPEN: &str = "<mjml>";
const WRAPPER_CLOSE: &str = "</mjml>";

#[inline(always)]
fn parse_attributes(cursor: &mut MrmlCursor<'_>) -> Result<MjmlAttributes, Error> {
    let mut attrs = MjmlAttributes::default();
    while let Some(token) = cursor.next_attribute()? {
        match token.local.as_str() {
            "owa" => attrs.owa = token.value.map(|v| v.to_string()),
            "lang" => attrs.lang = token.value.map(|v| v.to_string()),
            "dir" => attrs.dir = token.value.map(|v| v.to_string()),
            _ => cursor.add_warning(WarningKind::UnexpectedAttribute, token.span),
        }
    }
    Ok(attrs)
}

impl ParseAttributes<MjmlAttributes> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<MjmlAttributes, Error> {
        parse_attributes(cursor)
    }
}

fn parse_include_path(
    cursor: &mut MrmlCursor<'_>,
    tag_span: &htmlparser::StrSpan<'_>,
) -> Result<String, Error> {
    let mut path = None;
    while let Some(attr) = cursor.next_attribute()? {
        match attr.local.as_str() {
            "path" => path = attr.value.map(|v| v.to_string()),
            _ => cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span),
        }
    }
    path.ok_or_else(|| Error::MissingAttribute {
        name: "path",
        origin: cursor.origin(),
        position: tag_span.into(),
    })
}

fn merge_include_children(children: &mut MjmlChildren, included: MjmlChildren) {
    if let Some(head) = included.head {
        children.head = Some(head);
    }
    if let Some(body) = included.body {
        children.body = Some(body);
    }
}

impl ParseChildren<MjmlChildren> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<MjmlChildren, Error> {
        let mut children = MjmlChildren::default();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementClose(close) if close.local.as_str() == super::NAME => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                MrmlToken::Text(inner) if inner.text.trim().is_empty() => {
                    // ignoring empty text
                }
                MrmlToken::Comment(_) => {
                    // ignoring comment on purpose
                }
                MrmlToken::ElementStart(start) => match start.local.as_str() {
                    MJ_HEAD => {
                        children.head = Some(self.parse(cursor, start.local)?);
                    }
                    MJ_BODY => {
                        children.body = Some(self.parse(cursor, start.local)?);
                    }
                    MJ_INCLUDE => {
                        let path = parse_include_path(cursor, &start.local)?;
                        let ending = cursor.assert_element_end()?;
                        if !ending.empty {
                            cursor.assert_element_close()?;
                        }
                        let content =
                            self.options
                                .include_loader
                                .resolve(&path)
                                .map_err(|source| Error::IncludeLoaderError {
                                    origin: cursor.origin(),
                                    position: start.span.into(),
                                    source,
                                })?;
                        let wrapped = format!("{WRAPPER_OPEN}{content}{WRAPPER_CLOSE}");
                        let offset = WRAPPER_OPEN.len();
                        let with_position = |err: Error| err.adjust_positions(offset);
                        let mut sub = cursor.new_child(&path, wrapped.as_str());
                        sub.set_source_offset(offset);
                        sub.assert_element_start().map_err(&with_position)?;
                        sub.assert_element_end().map_err(&with_position)?;
                        let included: MjmlChildren =
                            self.parse_children(&mut sub).map_err(&with_position)?;
                        sub.assert_element_close().map_err(&with_position)?;
                        cursor.with_warnings(sub.warnings());
                        merge_include_children(&mut children, included);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: start.span.into(),
                        });
                    }
                },
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjmlAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        _tag: &StrSpan<'_>,
    ) -> Result<MjmlAttributes, Error> {
        parse_attributes(cursor)
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<MjmlChildren> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<MjmlChildren, Error> {
        let mut children = MjmlChildren::default();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementClose(close) if close.local.as_str() == super::NAME => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(children);
                }
                MrmlToken::Text(inner) if inner.text.trim().is_empty() => {
                    // ignoring empty text
                }
                MrmlToken::Comment(_) => {
                    // ignoring comment on purpose
                }
                MrmlToken::ElementStart(start) => match start.local.as_str() {
                    MJ_HEAD => {
                        children.head = Some(self.async_parse(cursor, start.local).await?);
                    }
                    MJ_BODY => {
                        children.body = Some(self.async_parse(cursor, start.local).await?);
                    }
                    MJ_INCLUDE => {
                        let path = parse_include_path(cursor, &start.local)?;
                        let ending = cursor.assert_element_end()?;
                        if !ending.empty {
                            cursor.assert_element_close()?;
                        }
                        let content = self
                            .options
                            .include_loader
                            .async_resolve(&path)
                            .await
                            .map_err(|source| Error::IncludeLoaderError {
                                origin: cursor.origin(),
                                position: start.span.into(),
                                source,
                            })?;
                        let wrapped = format!("{WRAPPER_OPEN}{content}{WRAPPER_CLOSE}");
                        let offset = WRAPPER_OPEN.len();
                        let with_position = |err: Error| err.adjust_positions(offset);
                        let mut sub = cursor.new_child(&path, wrapped.as_str());
                        sub.set_source_offset(offset);
                        sub.assert_element_start().map_err(&with_position)?;
                        sub.assert_element_end().map_err(&with_position)?;
                        let included: MjmlChildren = self
                            .async_parse_children(&mut sub)
                            .await
                            .map_err(&with_position)?;
                        sub.assert_element_close().map_err(&with_position)?;
                        cursor.with_warnings(sub.warnings());
                        merge_include_children(&mut children, included);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: start.span.into(),
                        });
                    }
                },
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
    }
}

impl Mjml {
    /// Function to parse a raw mjml template with some parsing
    /// [options](crate::prelude::parser::ParserOptions).
    ///
    /// You can specify the kind of loader mrml needs to use for loading the
    /// content of [`mj-include`](crate::mj_include) elements.
    ///
    /// You can take a look at the available loaders
    /// [here](crate::prelude::parser).
    ///
    /// ```rust
    /// use mrml::mjml::Mjml;
    /// use mrml::prelude::parser::ParserOptions;
    /// use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
    ///
    /// let options = ParserOptions {
    ///     include_loader: Box::new(MemoryIncludeLoader::default()),
    /// };
    /// match Mjml::parse_with_options("<mjml><mj-head /><mj-body /></mjml>", &options) {
    ///     Ok(_) => println!("Success!"),
    ///     Err(err) => eprintln!("Something went wrong: {err:?}"),
    /// }
    /// ```
    pub fn parse_with_options<T: AsRef<str>>(
        value: T,
        opts: &ParserOptions,
    ) -> Result<ParseOutput<Self>, Error> {
        let parser = MrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        let element = parser.parse_root(&mut cursor)?;
        Ok(ParseOutput {
            element,
            warnings: cursor.warnings(),
        })
    }

    #[cfg(feature = "async")]
    pub async fn async_parse_with_options<T: AsRef<str>>(
        value: T,
        opts: std::sync::Arc<crate::prelude::parser::AsyncParserOptions>,
    ) -> Result<ParseOutput<Self>, Error> {
        let parser = AsyncMrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        let element = parser.parse_root(&mut cursor).await?;
        Ok(ParseOutput {
            element,
            warnings: cursor.warnings(),
        })
    }

    /// Function to parse a raw mjml template using the default parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub fn parse<T: AsRef<str>>(value: T) -> Result<ParseOutput<Self>, Error> {
        let opts = ParserOptions::default();
        let parser = MrmlParser::new(&opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        let element = parser.parse_root(&mut cursor)?;
        Ok(ParseOutput {
            element,
            warnings: cursor.warnings(),
        })
    }

    #[cfg(feature = "async")]
    /// Function to parse a raw mjml template using the default parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub async fn async_parse<T: AsRef<str>>(value: T) -> Result<ParseOutput<Self>, Error> {
        let parser = AsyncMrmlParser::default();
        let mut cursor = MrmlCursor::new(value.as_ref());
        let element = parser.parse_root(&mut cursor).await?;
        Ok(ParseOutput {
            element,
            warnings: cursor.warnings(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_with_options_sync() {
        let template = "<mjml></mjml>";
        let output = Mjml::parse_with_options(template, &Default::default()).unwrap();
        assert!(output.element.children.body.is_none());
        assert!(output.element.children.head.is_none());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn should_parse_with_options_async() {
        let template = "<mjml></mjml>";
        let output = Mjml::async_parse_with_options(template, Default::default())
            .await
            .unwrap();
        assert!(output.element.children.body.is_none());
        assert!(output.element.children.head.is_none());
    }

    #[test]
    fn should_parse_sync() {
        let template = "<mjml></mjml>";
        let output = Mjml::parse(template).unwrap();
        assert!(output.element.children.body.is_none());
        assert!(output.element.children.head.is_none());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn should_parse_async() {
        let template = "<mjml></mjml>";
        let output = Mjml::async_parse(template).await.unwrap();
        assert!(output.element.children.body.is_none());
        assert!(output.element.children.head.is_none());
    }

    #[test]
    fn should_parse_without_children_sync() {
        let template = "<mjml />";
        let output: ParseOutput<Mjml> = Mjml::parse(template).unwrap();
        assert!(output.element.children.body.is_none());
        assert!(output.element.children.head.is_none());
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn should_parse_without_children_async() {
        let template = "<mjml />";
        let output: ParseOutput<Mjml> = Mjml::async_parse(template).await.unwrap();
        assert!(output.element.children.body.is_none());
        assert!(output.element.children.head.is_none());
    }

    #[test]
    fn should_parse_with_lang_sync() {
        let template = "<mjml lang=\"fr\"></mjml>";
        let output = Mjml::parse(template).unwrap();
        assert_eq!(output.element.attributes.lang.unwrap(), "fr");
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn should_parse_with_lang_async() {
        let template = "<mjml lang=\"fr\"></mjml>";
        let output = Mjml::async_parse(template).await.unwrap();
        assert_eq!(output.element.attributes.lang.unwrap(), "fr");
    }

    #[test]
    fn should_parse_with_owa() {
        let template = "<mjml owa=\"desktop\"></mjml>";
        let output = Mjml::parse(template).unwrap();
        assert_eq!(output.element.attributes.owa.unwrap(), "desktop");
    }

    #[test]
    fn should_parse_with_dir() {
        let template = "<mjml dir=\"rtl\"></mjml>";
        let output = Mjml::parse(template).unwrap();
        assert_eq!(output.element.attributes.dir.unwrap(), "rtl");
    }

    #[test]
    fn should_not_fail_with_unknown_param() {
        let template = "<mjml unknown=\"true\"></mjml>";
        let _output = Mjml::parse(template).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "UnexpectedToken { origin: Root, position: Span { start: 6, end: 11 } }"
    )]
    fn should_fail_with_text_as_child() {
        let template = "<mjml>Hello</mjml>";
        let _ = Mjml::parse(template).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "UnexpectedElement { origin: Root, position: Span { start: 6, end: 10 } }"
    )]
    fn should_fail_with_other_child() {
        let template = "<mjml><div /></mjml>";
        let _ = Mjml::parse(template).unwrap();
    }
}
