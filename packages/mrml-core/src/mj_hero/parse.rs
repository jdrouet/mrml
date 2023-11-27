use xmlparser::StrSpan;

use super::MjHero;
use crate::prelude::parser::{AsyncParseElement, Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjHero> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjHero, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjHero {
            attributes,
            children,
        })
    }
}

#[async_trait::async_trait]
impl AsyncParseElement<MjHero> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjHero, Error> {
        let (attributes, children) = self.async_parse_attributes_and_children(cursor).await?;

        Ok(MjHero {
            attributes,
            children,
        })
    }
}
