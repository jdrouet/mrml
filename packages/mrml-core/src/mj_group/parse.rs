use xmlparser::StrSpan;

use super::MjGroup;
use crate::prelude::parser::{AsyncParseElement, Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjGroup> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjGroup, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjGroup {
            attributes,
            children,
        })
    }
}

#[async_trait::async_trait]
impl AsyncParseElement<MjGroup> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjGroup, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(MjGroup {
            attributes,
            children,
        })
    }
}
