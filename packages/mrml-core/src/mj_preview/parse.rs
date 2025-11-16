#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren};
use crate::{
    comment::Comment,
    mj_preview::MjPreviewChild,
    prelude::{
        parser::{Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren},
        OneOrMany,
    },
    text::Text,
};

impl ParseChildren<OneOrMany<MjPreviewChild>> for MrmlParser<'_> {
    fn parse_children(
        &self,
        cursor: &mut MrmlCursor<'_>,
    ) -> Result<OneOrMany<MjPreviewChild>, Error> {
        let mut children = Vec::new();
        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::Comment(inner) => {
                    children.push(MjPreviewChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::Text(inner) => {
                    children.push(MjPreviewChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(OneOrMany::Many(children));
                }
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
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AsyncParseChildren<OneOrMany<MjPreviewChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<OneOrMany<MjPreviewChild>, Error> {
        let mut children = Vec::new();
        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::Comment(inner) => {
                    children.push(MjPreviewChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::Text(inner) => {
                    children.push(MjPreviewChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(OneOrMany::Many(children));
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
    use crate::mj_preview::MjPreview;

    crate::should_sync_parse!(
        should_parse,
        MjPreview,
        "<mj-preview>Hello World!</mj-preview>"
    );
    crate::should_sync_parse!(should_parse_without_children, MjPreview, "<mj-preview />");
}
