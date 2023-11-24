use xmlparser::StrSpan;

use super::MjAccordionTitle;
use crate::prelude::parser::{ChildrenParser, ElementParser, Error, MrmlCursor};
use crate::text::Text;

impl<'a> ChildrenParser<'a, Vec<Text>> for MrmlCursor<'a> {
    fn parse_children(&mut self) -> Result<Vec<Text>, Error> {
        let mut result = Vec::new();

        while let Some(item) = self.next_text()? {
            if !item.text.trim().is_empty() {
                result.push(Text::from(item.text.as_str()));
            }
        }

        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjAccordionTitle> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionTitle, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjAccordionTitle {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionTitle;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn should_work_with_child_text() {
        let raw = "<mj-accordion-title>Hello</mj-accordion-title>";
        let _: MjAccordionTitle = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn should_work_with_no_children() {
        let raw = "<mj-accordion-title />";
        let _: MjAccordionTitle = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "EndOfStream")]
    fn should_error_with_no_closing() {
        let raw = "<mj-accordion-title>";
        let _: MjAccordionTitle = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
