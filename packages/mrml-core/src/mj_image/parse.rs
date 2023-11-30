use xmlparser::StrSpan;

use super::MjImage;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjImage, Error> {
    let attributes = parse_attributes_map(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjImage { attributes })
}

impl<'opts> ParseElement<MjImage> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjImage, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjImage> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjImage, Error> {
        parse(cursor)
    }
}
