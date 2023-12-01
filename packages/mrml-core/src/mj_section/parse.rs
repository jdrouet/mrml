use xmlparser::StrSpan;

use super::MjSection;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl<'opts> ParseElement<MjSection> for MrmlParser<'opts> {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjSection, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjSection {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl AsyncParseElement<MjSection> for AsyncMrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjSection, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor).await?;

        Ok(MjSection {
            attributes,
            children,
        })
    }
}
