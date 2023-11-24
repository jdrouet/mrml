use xmlparser::StrSpan;

use super::MjAttributesAll;
use crate::prelude::parser::{AttributesParser, ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjAttributesAll> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAttributesAll, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if !ending.empty {
            self.assert_element_close()?;
        }

        Ok(MjAttributesAll { attributes })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes_all::MjAttributesAll;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn parse_complete() {
        let raw = r#"<mj-all color="red" />"#;
        let _: MjAttributesAll = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
