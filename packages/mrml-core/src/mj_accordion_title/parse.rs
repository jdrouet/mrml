use xmlparser::StrSpan;

use super::MjAccordionTitle;
use crate::{
    prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser},
    text::Text,
};

impl<'a> ChildrenParser<'a, Vec<Text>> for MrmlParser<'a> {
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

impl<'a> ElementParser<'a, MjAccordionTitle> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionTitle, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjAccordionTitle {
                attributes,
                children: Default::default(),
            });
        }
        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjAccordionTitle {
            attributes,
            children,
        })
    }
}
