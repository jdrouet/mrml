use xmlparser::StrSpan;

use super::{MjCarousel, MjCarouselChild};
use crate::comment::Comment;
use crate::mj_carousel_image::NAME as MJ_CAROUSEL_IMAGE;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<Vec<MjCarouselChild>> for MrmlParser {
    fn parse_children<'a>(
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
                        result.push(
                            MjCarouselChild::MjCarouselImage(self.parse(cursor, inner.local)?)
                        );
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

impl ParseElement<MjCarousel> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjCarousel, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjCarousel {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_carousel::MjCarousel;

    crate::should_parse!(
        with_all_children,
        MjCarousel,
        r#"<mj-carousel>
    <!-- comment -->
    <mj-carousel-image />
</mj-carousel>
"#
    );

    crate::should_not_parse!(
        with_unexpected_child,
        MjCarousel,
        r#"<mj-carousel>
        <mj-text>Nope</mj-text>
    </mj-carousel>
"#
    );
}
