use xmlparser::StrSpan;

use super::{MjCarousel, MjCarouselChild};
use crate::comment::Comment;
use crate::mj_carousel_image::NAME as MJ_CAROUSEL_IMAGE;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
};

impl<'a> ChildrenParser<'a, Vec<MjCarouselChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjCarouselChild>, Error> {
        let mut result = Vec::new();

        loop {
            match self.assert_next()? {
                MrmlToken::Comment(inner) => {
                    result.push(MjCarouselChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    if inner.local.as_str() == MJ_CAROUSEL_IMAGE {
                        result.push(MjCarouselChild::MjCarouselImage(self.parse(inner.local)?));
                    } else {
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjCarousel> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjCarousel, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjCarousel {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjCarousel {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mj_carousel::MjCarousel, prelude::parser::MrmlParser};

    #[test]
    fn with_all_children() {
        let raw = r#"<mj-carousel>
    <!-- comment -->
    <mj-carousel-image />
</mj-carousel>
"#;
        let _: MjCarousel = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn with_unexpected_child() {
        let raw = r#"<mj-carousel>
    <mj-text>Nope</mj-text>
</mj-carousel>
"#;
        let _: MjCarousel = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
