use xmlparser::StrSpan;

use super::MjCarouselImage;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjCarouselImage, Error> {
    let attributes = parse_attributes_map(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }
    Ok(MjCarouselImage { attributes })
}

impl<'opts> ParseElement<MjCarouselImage> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjCarouselImage, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjCarouselImage> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjCarouselImage, Error> {
        parse(cursor)
    }
}
