use xmlparser::StrSpan;

use super::MjDivider;
use crate::prelude::parser::{parse_attributes_map, Error, MrmlCursor, MrmlParser, ParseElement};
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};

fn parse(cursor: &mut MrmlCursor<'_>) -> Result<MjDivider, Error> {
    let attributes = parse_attributes_map(cursor)?;
    let ending = cursor.assert_element_end()?;
    if !ending.empty {
        cursor.assert_element_close()?;
    }

    Ok(MjDivider { attributes })
}

impl<'opts> ParseElement<MjDivider> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjDivider, Error> {
        parse(cursor)
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjDivider> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjDivider, Error> {
        parse(cursor)
    }
}
