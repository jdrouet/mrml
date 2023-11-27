use xmlparser::StrSpan;

use super::MjAccordionText;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjAccordionText> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionText, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjAccordionText {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionText;

    crate::should_parse!(
        should_work_with_child_text,
        MjAccordionText,
        "<mj-accordion-text>Hello</mj-accordion-text>"
    );

    crate::should_parse!(
        should_work_with_no_children,
        MjAccordionText,
        "<mj-accordion-text />"
    );

    crate::should_not_parse!(
        should_error_with_no_closing,
        MjAccordionText,
        "<mj-accordion-text>",
        "EndOfStream"
    );
}
