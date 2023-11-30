use xmlparser::StrSpan;

use super::MjColumn;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjColumn> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjColumn, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjColumn {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjColumn> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjColumn, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjColumn {
            attributes,
            children,
        })
    }
}
