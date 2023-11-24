use xmlparser::StrSpan;

use super::{MjAccordionElement, MjAccordionElementChildren};
use crate::mj_accordion_text::NAME as MJ_ACCORDION_TEXT;
use crate::mj_accordion_title::NAME as MJ_ACCORDION_TITLE;
use crate::prelude::parser::{ChildrenParser, ElementParser, Error, MrmlCursor, MrmlToken};

impl<'a> ChildrenParser<'a, MjAccordionElementChildren> for MrmlCursor<'a> {
    fn parse_children(&mut self) -> Result<MjAccordionElementChildren, Error> {
        let mut result = MjAccordionElementChildren::default();

        loop {
            let token = self.assert_next()?;
            match token {
                MrmlToken::ElementStart(inner) => match inner.local.as_str() {
                    MJ_ACCORDION_TEXT => {
                        result.text = Some(self.parse(inner.local)?);
                    }
                    MJ_ACCORDION_TITLE => {
                        result.title = Some(self.parse(inner.local)?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                },
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

impl<'a> ElementParser<'a, MjAccordionElement> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionElement, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjAccordionElement {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionElement;
    use crate::mj_accordion_text::MjAccordionText;
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn should_work_with_no_children() {
        let raw = "<mj-accordion-element />";
        let _: MjAccordionElement = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedElement(Span { start: 22, end: 27 })")]
    fn should_error_with_unknown_child() {
        let raw = "<mj-accordion-element><span /></mj-accordion-element>";
        let _: MjAccordionElement = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedToken(Span { start: 22, end: 38 }")]
    fn should_error_with_comment() {
        let raw = "<mj-accordion-element><!-- comment --></mj-accordion-element>";
        let _: MjAccordionElement = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn title_should_work_with_no_children() {
        let raw = "<mj-accordion-title />";
        let _: MjAccordionTitle = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn title_should_work_with_child_text() {
        let raw = "<mj-accordion-title>Hello</mj-accordion-title>";
        let _: MjAccordionTitle = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedToken(Span { start: 20, end: 25 })")]
    fn title_should_error_with_span_child() {
        let raw = "<mj-accordion-title><span>Hello</span></mj-accordion-title>";
        let _: MjAccordionTitle = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn text_should_work_with_child_text() {
        let raw = "<mj-accordion-text>Hello</mj-accordion-text>";
        let _: MjAccordionText = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "EndOfStream")]
    fn text_should_error_with_no_closing() {
        let raw = "<mj-accordion-text>";
        let _: MjAccordionText = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
