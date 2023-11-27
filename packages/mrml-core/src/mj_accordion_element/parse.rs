use xmlparser::StrSpan;

use super::{MjAccordionElement, MjAccordionElementChildren};
use crate::mj_accordion_text::NAME as MJ_ACCORDION_TEXT;
use crate::mj_accordion_title::NAME as MJ_ACCORDION_TITLE;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseChildren<MjAccordionElementChildren> for MrmlParser {
    fn parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<MjAccordionElementChildren, Error> {
        let mut result = MjAccordionElementChildren::default();

        loop {
            let token = cursor.assert_next()?;
            match token {
                MrmlToken::ElementStart(inner) => match inner.local.as_str() {
                    MJ_ACCORDION_TEXT => {
                        result.text = Some(self.parse(cursor, inner.local)?);
                    }
                    MJ_ACCORDION_TITLE => {
                        result.title = Some(self.parse(cursor, inner.local)?);
                    }
                    _ => {
                        return Err(Error::UnexpectedElement(inner.span.into()));
                    }
                },
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
    }
}

impl ParseElement<MjAccordionElement> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAccordionElement, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjAccordionElement {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjAccordionElement;

    crate::should_parse!(
        should_work_with_no_children,
        MjAccordionElement,
        "<mj-accordion-element />"
    );

    crate::should_not_parse!(
        should_error_with_unknown_child,
        MjAccordionElement,
        "<mj-accordion-element><span /></mj-accordion-element>",
        "UnexpectedElement(Span { start: 22, end: 27 })"
    );

    crate::should_not_parse!(
        should_error_with_comment,
        MjAccordionElement,
        "<mj-accordion-element><!-- comment --></mj-accordion-element>",
        "UnexpectedToken(Span { start: 22, end: 38 }"
    );
}
