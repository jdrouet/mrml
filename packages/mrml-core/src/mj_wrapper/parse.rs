use xmlparser::StrSpan;

use super::MjWrapper;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjWrapper> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjWrapper, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjWrapper {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MjWrapper;

    crate::should_sync_parse!(
        parse_br_element,
        MjWrapper,
        "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>"
    );
}
