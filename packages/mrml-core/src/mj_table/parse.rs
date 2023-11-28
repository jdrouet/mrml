use xmlparser::StrSpan;

use super::MjTable;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjTable> for MrmlParser {
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
impl crate::prelude::parser::AsyncParseElement<MjTable> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjTable, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(MjTable {
            attributes,
            children,
        })
    }
}
