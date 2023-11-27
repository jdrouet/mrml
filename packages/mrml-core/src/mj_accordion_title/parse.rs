use xmlparser::StrSpan;

use super::MjAccordionTitle;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseChildren, ParseElement};
use crate::text::Text;

impl ParseChildren<Vec<Text>> for MrmlParser {
    fn parse_children<'a>(&self, cursor: &mut MrmlCursor<'a>) -> Result<Vec<Text>, Error> {
        let mut result = Vec::new();

        while let Some(item) = cursor.next_text()? {
            if !item.text.trim().is_empty() {
                result.push(Text::from(item.text.as_str()));
            }
        }

        Ok(result)
    }
}

impl ParseElement<MjAccordionTitle> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionTitle, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjAccordionTitle {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionTitle;

    crate::should_parse!(
        should_work_with_child_text,
        MjAccordionTitle,
        "<mj-accordion-title>Hello</mj-accordion-title>"
    );

    crate::should_parse!(
        should_work_with_no_children,
        MjAccordionTitle,
        "<mj-accordion-title />"
    );

    crate::should_not_parse!(
        should_error_with_no_closing,
        MjAccordionTitle,
        "<mj-accordion-title>",
        "EndOfStream"
    );
}
