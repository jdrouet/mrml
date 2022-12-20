use super::MJCarouselImage;
use crate::parse_attribute;
use crate::prelude::parse::{Error, Parsable, Parser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug, Default)]
struct MJCarouselImageParser(MJCarouselImage);

impl Parser for MJCarouselImageParser {
    type Output = MJCarouselImage;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
}

impl Parsable for MJCarouselImage {
    fn parse(_tag: StrSpan, tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJCarouselImageParser::default().parse(tokenizer)?.build()
    }
}
