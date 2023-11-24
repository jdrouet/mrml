use xmlparser::StrSpan;

use super::MjWrapper;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjWrapper> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjWrapper, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjWrapper {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MjWrapper;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn parse_br_element() {
        let content = "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>";
        let _: MjWrapper = MrmlCursor::new(content, Default::default())
            .parse_root()
            .unwrap();
    }
}
