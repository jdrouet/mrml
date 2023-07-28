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
            result.push(Text::from(item.text.as_str()));
        }

        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjAccordionTitle> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAccordionTitle, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.next_element_end()?.ok_or(Error::EndOfStream)?;
        let children = if !ending.empty {
            self.parse_children()?
        } else {
            Default::default()
        };

        Ok(MjAccordionTitle {
            attributes,
            children,
        })
    }
}
