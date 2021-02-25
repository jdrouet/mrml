use super::children::MJAttributesChild;
use super::MJAttributes;
use crate::elements::head::{mj_attributes_all, mj_attributes_class, mj_attributes_element};
use crate::parser::{Error, MJMLParser};
use xmlparser::{StrSpan, Tokenizer};

#[derive(Default)]
struct MJAttributesParser {
    children: Vec<MJAttributesChild>,
}

impl MJMLParser for MJAttributesParser {
    type Output = MJAttributes;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJAttributes {
            children: self.children,
        })
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer,
    ) -> Result<(), Error> {
        self.children.push(match tag.as_str() {
            mj_attributes_all::NAME => mj_attributes_all::MJAttributesAll::parse(tokenizer)?.into(),
            mj_attributes_class::NAME => {
                mj_attributes_class::MJAttributesClass::parse(tokenizer)?.into()
            }
            _ => mj_attributes_element::MJAttributesElement::parse(tag, tokenizer)?.into(),
        });
        Ok(())
    }
}

impl MJAttributes {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJAttributesParser::default().parse(tokenizer)?.build()
    }
}
