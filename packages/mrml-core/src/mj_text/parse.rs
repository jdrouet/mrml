use xmlparser::StrSpan;

use super::MjText;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjText> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _tag: StrSpan<'a>) -> Result<MjText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjText {
            attributes,
            children,
        })
    }
}

#[cfg(feature = "async")]
#[async_trait::async_trait(?Send)]
impl crate::prelude::parser::AsyncParseElement<MjText> for MrmlParser {
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

#[cfg(test)]
mod tests {
    use crate::mj_text::MjText;

    crate::should_parse!(self_closing, MjText, "<mj-text />");
    crate::should_parse!(normal, MjText, "<mj-text>Hello World!</mj-text>");
}
