use xmlparser::StrSpan;

use super::MjTable;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjTable> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjTable, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjTable {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjTable> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjTable, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjTable {
            attributes,
            children,
        })
    }
}
