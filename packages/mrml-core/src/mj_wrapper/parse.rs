use xmlparser::StrSpan;

use super::MjWrapper;
use crate::prelude::parser::{AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjWrapper> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjWrapper, Error> {
        let attributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjWrapper {
                attributes,
                children: Vec::new(),
            });
        }

        let children = self.parse_children()?;

        self.assert_element_close()?;

        Ok(MjWrapper {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::mj_wrapper::MjWrapper;
    use crate::prelude::parser::{Parsable, ParserOptions};

    #[test]
    fn parse_br_element() {
        let opts = Rc::new(ParserOptions::default());
        let content = "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>";
        let mut tokenizer = xmlparser::Tokenizer::from(content);
        let _ = tokenizer.next().unwrap();
        let tag = xmlparser::StrSpan::from("<mj-wrapper");
        MjWrapper::parse(tag, &mut tokenizer, opts).unwrap();
    }
}
