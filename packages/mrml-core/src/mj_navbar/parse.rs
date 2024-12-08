use super::MjNavbarChild;
use crate::comment::Comment;
use crate::mj_navbar_link::NAME as MJ_NAVBAR_LINK;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjNavbarChild>> for MrmlParser<'_> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjNavbarChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjNavbarChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_NAVBAR_LINK {
                        result.push(MjNavbarChild::MjNavbarLink(
                            self.parse(cursor, inner.local)?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
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
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjNavbarChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjNavbarChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjNavbarChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_NAVBAR_LINK {
                        result.push(MjNavbarChild::MjNavbarLink(
                            self.async_parse(cursor, inner.local).await?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement {
                            origin: cursor.origin(),
                            position: inner.span.into(),
                        });
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
    use crate::mj_navbar::MjNavbar;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            crate::should_sync_parse!($title, MjNavbar, $template);
        };
    }

    macro_rules! assert_fail {
        ($title:ident, $template:expr, $error:expr) => {
            crate::should_not_sync_parse!($title, MjNavbar, $template, $error);
        };
    }

    assert_success!(should_handle_empty_children, "<mj-navbar />");

    assert_success!(
        should_handle_comments,
        "<mj-navbar><!-- comment --></mj-navbar>"
    );

    assert_fail!(
        should_error_with_text,
        "<mj-navbar>Hello</mj-navbar>",
        "UnexpectedToken { origin: Root, position: Span { start: 11, end: 16 } }"
    );

    assert_fail!(
        should_error_with_other_element,
        "<mj-navbar><span /></mj-navbar>",
        "UnexpectedElement { origin: Root, position: Span { start: 11, end: 16 } }"
    );
}
