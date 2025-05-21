use super::RootChild;
use crate::comment::Comment;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseOutput, ParserOptions, WarningKind,
};

#[inline]
fn skip_element_and_children(cursor: &mut MrmlCursor<'_>) -> Result<(), Error> {
    // The initial ElementStart for the unknown tag is already consumed by the caller.
    // We now need to consume its attributes (if any) and then its children or the closing tag.

    // Consume any attributes associated with the unknown tag.
    while cursor.next_attribute()?.is_some() {
        // just consume
    }

    // Check if the element is self-closing (e.g. <mj-unknown />)
    // We use assert_element_end which expects either an ElementEndToken::Empty or ElementEndToken::Close
    // This also consumes the token.
    let ending_token = cursor.assert_element_end()?;
    if ending_token.empty {
        return Ok(()); // Self-closing, nothing more to skip.
    }

    // Not self-closing, so we need to skip children until the corresponding closing tag.
    let mut depth = 1;
    while depth > 0 {
        match cursor.next_token()? {
            Some(MrmlToken::ElementStart(_element_start_token)) => {
                // Consume attributes of this nested element
                while cursor.next_attribute()?.is_some() {}
                // Check if this nested element is self-closing
                let nested_ending = cursor.assert_element_end()?;
                if !nested_ending.empty {
                    depth += 1;
                }
            }
            Some(MrmlToken::ElementClose(_element_close_token)) => {
                depth -= 1;
            }
            Some(MrmlToken::Comment(_)) | Some(MrmlToken::Text(_)) => { /* Consume other tokens */ }
            None => {
                // This means an unclosed tag while skipping, which is an error.
                return Err(Error::UnexpectedEofKind {
                    kind: "element".into(),
                    position: cursor.span().into(),
                    origin: cursor.origin(),
                });
            }
            // Any other token type encountered during skipping that isn't explicitly handled above
            // (like ElementEnd which is handled by assert_element_end) would be an issue.
            // However, next_token() should cover the main cases.
            // If an unexpected token type that breaks the structure appears, it might indicate
            // a problem with the input MJML or the tokenization process itself.
            // For robustness, we can treat unexpected tokens during skipping as an error.
            Some(other) => {
                return Err(Error::UnexpectedToken {
                    origin: cursor.origin(),
                    position: other.span(),
                });
            }
        }
    }
    Ok(())
}

impl crate::prelude::parser::ParseChildren<Vec<RootChild>> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<RootChild>, Error> {
        use crate::prelude::parser::ParseElement;

        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(RootChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.eq("mjml") {
                        result.push(RootChild::Mjml(self.parse(cursor, inner.local)?));
                    } else {
                        cursor.add_warning(WarningKind::UnknownElement, inner.span.into());
                        skip_element_and_children(cursor)?;
                    }
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MjBody;
    use crate::mj_text::MjText;
    use crate::mjml::Mjml;
    use crate::prelude::parser::{ParserOptions, WarningKind};
    use crate::root::{Root, RootChild};

    #[test]
    fn should_warn_and_skip_single_unknown_tag() {
        let mrml = r#"
            <mj-unknown></mj-unknown>
            <mjml>
                <mj-body>
                    <mj-text>Hello World</mj-text>
                </mj-body>
            </mjml>
        "#;
        let opts = ParserOptions::default();
        let result = Root::parse_with_options(mrml, &opts);

        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.warnings.len(), 1);
        assert_eq!(output.warnings[0].kind, WarningKind::UnknownElement);
        assert!(output.warnings[0].to_string().contains("unknown element"));
        assert!(output.warnings[0].to_string().contains("<mj-unknown>"));

        assert_eq!(output.element.0.len(), 1);
        match &output.element.0[0] {
            RootChild::Mjml(mjml_node) => {
                assert_eq!(mjml_node.children.len(), 1);
                if let Some(crate::mjml::MjmlChild::MjBody(mj_body)) = &mjml_node.children.get(0) {
                    assert_eq!(mj_body.children.len(), 1);
                    if let Some(crate::mj_body::MjBodyChild::MjText(mj_text)) = &mj_body.children.get(0) {
                        assert_eq!(mj_text.content, "Hello World");
                    } else {
                        panic!("Expected MjText");
                    }
                } else {
                    panic!("Expected MjBody");
                }
            }
            _ => panic!("Expected RootChild::Mjml"),
        }
    }

    #[test]
    fn should_warn_and_skip_unknown_tag_with_children() {
        let mrml = r#"
            <mj-unknown>
                <mj-foo>
                    <mj-bar />
                </mj-foo>
            </mj-unknown>
            <mjml>
                <mj-body>
                    <mj-text>Valid Content</mj-text>
                </mj-body>
            </mjml>
        "#;
        let opts = ParserOptions::default();
        let result = Root::parse_with_options(mrml, &opts);

        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.warnings.len(), 1);
        assert_eq!(output.warnings[0].kind, WarningKind::UnknownElement);
        assert!(output.warnings[0].to_string().contains("<mj-unknown>"));

        assert_eq!(output.element.0.len(), 1); // Only mjml should be present
        match &output.element.0[0] {
            RootChild::Mjml(mjml_node) => {
                 if let Some(crate::mjml::MjmlChild::MjBody(mj_body)) = &mjml_node.children.get(0) {
                    if let Some(crate::mj_body::MjBodyChild::MjText(mj_text)) = &mj_body.children.get(0) {
                        assert_eq!(mj_text.content, "Valid Content");
                    } else {
                        panic!("Expected MjText");
                    }
                } else {
                    panic!("Expected MjBody");
                }
            }
            _ => panic!("Expected RootChild::Mjml, found {:?}", output.element.0[0]),
        }
    }

    #[test]
    fn should_warn_and_skip_unknown_self_closing_tag() {
        let mrml = r#"
            <mj-unknown-self-closing />
            <mjml>
                <mj-body>
                    <mj-text>Another Content</mj-text>
                </mj-body>
            </mjml>
        "#;
        let opts = ParserOptions::default();
        let result = Root::parse_with_options(mrml, &opts);

        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.warnings.len(), 1);
        assert_eq!(output.warnings[0].kind, WarningKind::UnknownElement);
        assert!(output.warnings[0].to_string().contains("<mj-unknown-self-closing />"));

        assert_eq!(output.element.0.len(), 1); // Only mjml
        match &output.element.0[0] {
            RootChild::Mjml(mjml_node) => {
                // Basic check for MjBody and MjText presence
                assert!(!mjml_node.children.is_empty());
            }
            _ => panic!("Expected RootChild::Mjml"),
        }
    }

    #[test]
    fn should_handle_mixed_valid_and_unknown_tags() {
        let mrml = r#"
            <mj-unknown-before />
            <mjml>
              <mj-body>
                <mj-text>Hello</mj-text>
              </mj-body>
            </mjml>
            <mj-unknown-after>
                <mj-nested />
            </mj-unknown-after>
        "#;
        let opts = ParserOptions::default();
        let result = Root::parse_with_options(mrml, &opts);

        assert!(result.is_ok());
        let output = result.unwrap();

        assert_eq!(output.warnings.len(), 2);
        assert_eq!(output.warnings[0].kind, WarningKind::UnknownElement);
        assert!(output.warnings[0].to_string().contains("<mj-unknown-before />"));
        assert_eq!(output.warnings[1].kind, WarningKind::UnknownElement);
        assert!(output.warnings[1].to_string().contains("<mj-unknown-after>"));
        
        assert_eq!(output.element.0.len(), 1); // Only mjml
        match &output.element.0[0] {
            RootChild::Mjml(mjml_node) => {
                if let Some(crate::mjml::MjmlChild::MjBody(mj_body)) = &mjml_node.children.get(0) {
                    if let Some(crate::mj_body::MjBodyChild::MjText(mj_text)) = &mj_body.children.get(0) {
                        assert_eq!(mj_text.content, "Hello");
                    } else {
                        panic!("Expected MjText");
                    }
                } else {
                    panic!("Expected MjBody");
                }
            }
            _ => panic!("Expected RootChild::Mjml"),
        }
    }

    #[test]
    fn should_still_error_on_malformed_unclosed_tags() {
        let mrml = r#"
            <mjml>
                <mj-body>
                    <mj-text>Valid</mj-text>
                </mj-body>
            </mjml>
            <mj-unclosed
        "#;
        let opts = ParserOptions::default();
        let result = Root::parse_with_options(mrml, &opts);

        assert!(result.is_err());
        let error = result.err().unwrap();
        // Expecting an UnexpectedEofKind or similar, depending on how the parser handles premature EOF.
        // The exact error might be Error::UnexpectedEofKind or Error::UnexpectedToken if it hits EOF
        // while expecting more attributes or the end of the tag.
        match error {
            crate::prelude::parser::Error::UnexpectedEofKind { kind, .. } => {
                assert_eq!(kind, "element");
            }
            crate::prelude::parser::Error::UnexpectedToken { .. } => {
                // This could also be a valid outcome if EOF is treated as an unexpected token
                // when seeking the end of an attribute list or tag.
            }
            _ => panic!("Expected UnexpectedEofKind or UnexpectedToken error, got {:?}", error),
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl crate::prelude::parser::AsyncParseChildren<Vec<RootChild>>
    for crate::prelude::parser::AsyncMrmlParser
{
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<RootChild>, Error> {
        use crate::prelude::parser::AsyncParseElement;

        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(RootChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.eq("mjml") {
                        let element = self.async_parse(cursor, inner.local).await?;
                        result.push(RootChild::Mjml(element));
                    } else {
                        cursor.add_warning(WarningKind::UnknownElement, inner.span.into());
                        skip_element_and_children(cursor)?;
                    }
                }
                other => {
                    return Err(Error::UnexpectedToken {
                        origin: cursor.origin(),
                        position: other.span(),
                    });
                }
            }
        }
        Ok(result)
    }
}

impl super::Root {
    /// Function to parse a raw mjml template with some parsing
    /// [options](crate::prelude::parser::ParserOptions).
    pub(crate) fn parse_with_options<T: AsRef<str>>(
        value: T,
        opts: &ParserOptions,
    ) -> Result<ParseOutput<Self>, Error> {
        let parser = MrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        let element = Self(parser.parse_children(&mut cursor)?);
        Ok(ParseOutput {
            element,
            warnings: cursor.warnings(),
        })
    }

    #[cfg(feature = "async")]
    pub(crate) async fn async_parse_with_options<T: AsRef<str>>(
        value: T,
        opts: std::sync::Arc<crate::prelude::parser::AsyncParserOptions>,
    ) -> Result<ParseOutput<Self>, Error> {
        use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren};

        let parser = AsyncMrmlParser::new(opts);
        let mut cursor = MrmlCursor::new(value.as_ref());
        let element = Self(parser.async_parse_children(&mut cursor).await?);
        Ok(ParseOutput {
            element,
            warnings: cursor.warnings(),
        })
    }
}
