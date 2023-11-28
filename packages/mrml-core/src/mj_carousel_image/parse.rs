use xmlparser::StrSpan;

use super::MjCarouselImage;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, ParseElement};

impl ParseElement<MjCarouselImage> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjCarouselImage, Error> {
        let attributes = self.parse_attributes(cursor)?;
        let ending = cursor.assert_element_end()?;
        if !ending.empty {
            cursor.assert_element_close()?;
        }
        Ok(MjCarouselImage { attributes })
    }
}
