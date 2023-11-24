use xmlparser::StrSpan;

use super::MjCarouselImage;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjCarouselImage> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjCarouselImage, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
        }
        Ok(MjCarouselImage { attributes })
    }
}
