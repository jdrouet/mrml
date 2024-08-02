use super::MjCarouselChild;
use crate::comment::Comment;
use crate::mj_carousel_image::NAME as MJ_CAROUSEL_IMAGE;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl<'opts> ParseChildren<Vec<MjCarouselChild>> for MrmlParser<'opts> {
    fn parse_children(&self, cursor: &mut MrmlCursor<'_>) -> Result<Vec<MjCarouselChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjCarouselChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_CAROUSEL_IMAGE {
                        result.push(MjCarouselChild::MjCarouselImage(
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
impl AsyncParseChildren<Vec<MjCarouselChild>> for AsyncMrmlParser {
    async fn async_parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjCarouselChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjCarouselChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_CAROUSEL_IMAGE {
                        result.push(MjCarouselChild::MjCarouselImage(
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
    use crate::mj_carousel::MjCarousel;

    crate::should_sync_parse!(
        with_all_children,
        MjCarousel,
        r#"<mj-carousel>
    <!-- comment -->
    <mj-carousel-image />
</mj-carousel>
"#
    );

    crate::should_not_sync_parse!(
        with_unexpected_child,
        MjCarousel,
        r#"<mj-carousel>
        <mj-text>Nope</mj-text>
    </mj-carousel>
"#
    );
}
