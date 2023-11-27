use xmlparser::StrSpan;

use super::{MjAttributes, MjAttributesChild};
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::prelude::parser::{
    Error, MrmlCursor, MrmlParser, MrmlToken, ParseChildren, ParseElement,
};

impl ParseElement<MjAttributesChild> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjAttributesChild, Error> {
        Ok(match tag.as_str() {
            MJ_ALL => MjAttributesChild::MjAttributesAll(self.parse(cursor, tag)?),
            MJ_CLASS => MjAttributesChild::MjAttributesClass(self.parse(cursor, tag)?),
            _ => MjAttributesChild::MjAttributesElement(self.parse(cursor, tag)?),
        })
    }
}

impl ParseChildren<Vec<MjAttributesChild>> for MrmlParser {
    fn parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjAttributesChild>, Error> {
        let mut result = Vec::new();

        loop {
            match cursor.assert_next()? {
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(inner) => {
                    cursor.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }
    }
}

impl ParseElement<MjAttributes> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjAttributes, Error> {
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok(MjAttributes {
                children: Default::default(),
            });
        }

        let children = self.parse_children(cursor)?;
        cursor.assert_element_close()?;

        Ok(MjAttributes { children })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_attributes::MjAttributes;

    crate::should_parse!(
        parse_complete,
        MjAttributes,
        r#"
    <mj-attributes>
        <mj-all color="red" />
        <mj-class name="head" color="green" />
        <span color="blue" />
    </mj-attributes>
"#
    );
}
