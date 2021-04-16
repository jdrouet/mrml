use super::MJCarousel;
use super::MJCarouselChild;
use crate::prelude::parse::{Error, Parsable, Parser};
use crate::{parse_attribute, parse_child, parse_comment};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJCarouselParser(MJCarousel);

impl Parser for MJCarouselParser {
    type Output = MJCarousel;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJCarouselChild);
    parse_comment!();
}

impl Parsable for MJCarousel {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJCarouselParser::default().parse(tokenizer)?.build()
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
        assert!(crate::mjml::MJML::parse(json).is_ok());
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
        assert!(crate::mjml::MJML::parse(json).is_err());
    }
}
