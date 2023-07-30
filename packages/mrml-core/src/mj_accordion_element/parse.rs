use xmlparser::StrSpan;

use super::{MjAccordionElement, MjAccordionElementChildren};
use crate::mj_accordion_text::NAME as MJ_ACCORDION_TEXT;
use crate::mj_accordion_title::NAME as MJ_ACCORDION_TITLE;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
};

impl<'a> ChildrenParser<'a, MjAccordionElementChildren> for MrmlParser<'a> {
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
                        return Err(Error::UnexpectedElement(inner.span.start()));
                    }
                },
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => {
                    return Err(Error::unexpected_token(other.range()));
                }
            }
        }
    }
}

impl<'a> ElementParser<'a, MjAccordionElement> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionElement, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjAccordionElement {
                attributes,
                children: Default::default(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjAccordionElement {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion_text::MjAccordionText;
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::prelude::parser::{Error, MrmlParser};

    #[test]
    fn parse_title_child() {
        let raw = "<mj-accordion-title>Hello</mj-accordion-title>";
        let _: MjAccordionTitle = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn parse_title_child_errored() {
        let raw = "<mj-accordion-title><span>Hello</span></mj-accordion-title>";
        let res: Result<MjAccordionTitle, Error> =
            MrmlParser::new(raw, Default::default()).parse_root();
        assert!(matches!(res.unwrap_err(), Error::UnexpectedToken(20, 25)));
    }

    #[test]
    fn parse_text_child() {
        let raw = "<mj-accordion-text>Hello</mj-accordion-text>";
        let _: MjAccordionText = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn parse_text_child_errored() {
        let raw = "<mj-accordion-text>";
        let res: Result<MjAccordionText, Error> =
            MrmlParser::new(raw, Default::default()).parse_root();
        assert!(matches!(res.unwrap_err(), Error::EndOfStream));
    }
}
