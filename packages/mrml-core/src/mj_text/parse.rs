use xmlparser::StrSpan;

use super::MjText;
use crate::prelude::parser::{AsyncParseElement, Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjText> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjText {
            attributes,
            children,
        })
    }
}

#[async_trait::async_trait]
impl AsyncParseElement<MjText> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjText, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(MjText {
            attributes,
            children,
        })
    }
}
