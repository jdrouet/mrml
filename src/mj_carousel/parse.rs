use std::rc::Rc;

use super::MjCarouselChild;
use crate::mj_carousel_image::{MjCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
use crate::prelude::parse::{Error, Parsable, ParserOptions};
use xmlparser::{StrSpan, Tokenizer};

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
