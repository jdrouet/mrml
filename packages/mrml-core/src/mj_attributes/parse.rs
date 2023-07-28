use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjAttributes, MjAttributesChild};
use crate::mj_attributes_all::{MjAttributesAll, NAME as MJ_ALL};
use crate::mj_attributes_class::{MjAttributesClass, NAME as MJ_CLASS};
use crate::mj_attributes_element::MjAttributesElement;
use crate::prelude::parser::{
    ChildrenParser, ElementParser, Error, MrmlParser, Parsable, ParserOptions,
};

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

        while let Some(start) = self.next_element_start()? {
            result.push(self.parse(start.local)?);
        }

        Ok(result)
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

impl Parsable for MjAttributesChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ALL => Ok(MjAttributesAll::parse(tag, tokenizer, opts)?.into()),
            MJ_CLASS => Ok(MjAttributesClass::parse(tag, tokenizer, opts)?.into()),
            _ => Ok(MjAttributesElement::parse(tag, tokenizer, opts)?.into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

    #[test]
    fn parse_complete() {
        let template = r#"
        <mjml>
            <mj-head>
                <mj-attributes>
                    <mj-all color="red" />
                    <mj-class name="head" color="green" />
                    <span color="blue" />
                </mj-attributes>
            </mj-head>
        </mjml>
        "#;
        let elt = Mjml::parse(template).unwrap();
        assert!(elt.head().is_some());
        assert!(elt.body().is_none());
        let head = elt.head().unwrap();
        assert_eq!(head.children().len(), 1);
    }
}
