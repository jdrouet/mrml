use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjCarousel, MjCarouselChild};
use crate::comment::Comment;
use crate::mj_carousel_image::{MjCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken, Parsable,
    ParserOptions,
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
                        return Err(Error::UnexpectedElement(inner.span.start()));
                    }
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::unexpected_token(other.range())),
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

impl Parsable for MjCarouselChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_CAROUSEL_IMAGE => Ok(MjCarouselImage::parse(tag, tokenizer, opts)?.into()),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn with_all_children() {
        let json = r#"<mjml>
  <mj-body>
    <mj-carousel>
      <!-- comment -->
      <mj-carousel-image />
    </mj-carousel>
  </mj-body>
</mjml>
"#;
        assert!(crate::mjml::Mjml::parse(json).is_ok());
    }

    #[test]
    fn with_unexpected_child() {
        let json = r#"<mjml>
  <mj-body>
    <mj-carousel>
      <mj-text>Nope</mj-text>
    </mj-carousel>
  </mj-body>
</mjml>
"#;
        assert!(crate::mjml::Mjml::parse(json).is_err());
    }
}
