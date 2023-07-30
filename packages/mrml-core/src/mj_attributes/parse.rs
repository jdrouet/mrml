use xmlparser::StrSpan;

use super::{MjAttributes, MjAttributesChild};
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::prelude::parser::{ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken};

impl<'a> ElementParser<'a, MjAttributesChild> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjAttributesChild, Error> {
        Ok(match tag.as_str() {
            MJ_ALL => MjAttributesChild::MjAttributesAll(self.parse(tag)?),
            MJ_CLASS => MjAttributesChild::MjAttributesClass(self.parse(tag)?),
            _ => MjAttributesChild::MjAttributesElement(self.parse(tag)?),
        })
    }
}

impl<'a> ChildrenParser<'a, Vec<MjAttributesChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjAttributesChild>, Error> {
        let mut result = Vec::new();

        loop {
            match self.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(inner.local)?);
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjAttributes> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjAttributes, Error> {
        let ending = self.next_element_end()?.ok_or(Error::EndOfStream)?;
        if ending.empty {
            return Ok(MjAttributes {
                children: Default::default(),
            });
        }

        let children = self.parse_children()?;
        self.assert_element_close()?;

        Ok(MjAttributes { children })
    }
}

#[cfg(test)]
mod tests {
    use crate::{mj_attributes::MjAttributes, prelude::parser::MrmlParser};

    #[test]
    fn parse_complete() {
        let raw = r#"
<mj-attributes>
    <mj-all color="red" />
    <mj-class name="head" color="green" />
    <span color="blue" />
</mj-attributes>
        "#;
        let _: MjAttributes = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
