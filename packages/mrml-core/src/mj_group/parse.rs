use xmlparser::StrSpan;

use super::MjGroup;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjGroup> for MrmlParser<'opts> {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjGroup, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjGroup {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjGroup> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjGroup, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjGroup {
            attributes,
            children,
        })
    }
}
