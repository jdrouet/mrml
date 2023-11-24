use xmlparser::StrSpan;

use super::MjAccordionText;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjAccordionText> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionText, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjAccordionText {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionText;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn should_work_with_child_text() {
        let raw = "<mj-accordion-text>Hello</mj-accordion-text>";
        let _: MjAccordionText = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn should_work_with_no_children() {
        let raw = "<mj-accordion-text />";
        let _: MjAccordionText = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "EndOfStream")]
    fn should_error_with_no_closing() {
        let raw = "<mj-accordion-text>";
        let _: MjAccordionText = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
