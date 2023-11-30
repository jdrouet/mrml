use xmlparser::StrSpan;

use super::MjSpacer;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

#[inline]
fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjSpacer, Error> {
    let attributes = parse_attributes_map(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjSpacer { attributes })
}

impl<'opts> ParseElement<MjSpacer> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjSpacer, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjSpacer> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _: StrSpan<'a>,
    ) -> Result<MjSpacer, Error> {
        parse(cursor)
    }
}
