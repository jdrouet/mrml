use xmlparser::StrSpan;

use super::{MjSocial, MjSocialChild};
use crate::comment::Comment;
use crate::mj_social_element::NAME as MJ_SOCIAL_ELEMENT;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl<'opts> ParseChildren<Vec<MjSocialChild>> for MrmlParser<'opts> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjSocialChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjSocialChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_SOCIAL_ELEMENT {
                        result.push(MjSocialChild::MjSocialElement(
                            self.parse(cursor, inner.local)?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<Vec<MjSocialChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjSocialChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjSocialChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_SOCIAL_ELEMENT {
                        result.push(MjSocialChild::MjSocialElement(
                            self.async_parse(cursor, inner.local).await?,
                        ));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl<'opts> ParseElement<MjSocial> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjSocial, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjSocial {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseElement<MjSocial> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjSocial, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjSocial {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjSocial;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            crate::should_sync_parse!($title, MjSocial, $template);
        };
    }

    macro_rules! assert_fail {
        ($title:ident, $template:expr, $error:expr) => {
            crate::should_not_sync_parse!($title, MjSocial, $template, $error);
        };
    }

    assert_success!(should_handle_empty_children, "<mj-social />");

    assert_success!(
        should_handle_comments,
        "<mj-social><!-- comment --></mj-social>"
    );

    assert_fail!(
        should_error_with_text,
        "<mj-social>Hello</mj-social>",
        "UnexpectedToken(Span { start: 11, end: 16 })"
    );

    assert_fail!(
        should_error_with_other_element,
        "<mj-social><span /></mj-social>",
        "UnexpectedElement(Span { start: 11, end: 16 })"
    );
}
