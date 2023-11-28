use xmlparser::StrSpan;

use super::MjColumn;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjColumn> for MrmlParser {
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
impl crate::prelude::parser::AsyncParseElement<MjColumn> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjColumn, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(MjColumn {
            attributes,
            children,
        })
    }
}
