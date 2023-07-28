use xmlparser::StrSpan;

use super::MjImage;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjImage> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjImage, Error> {
        let attributes = self.parse_attributes()?;

        Ok(MjImage { attributes })
    }
}
