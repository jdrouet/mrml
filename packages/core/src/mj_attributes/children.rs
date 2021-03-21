use crate::from_child;
use crate::mj_attributes_all::MJAttributesAll;
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::MJAttributesClass;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::mj_attributes_element::MJAttributesElement;
use crate::prelude::parse::Error as ParserError;
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
pub enum MJAttributesChild {
    MJAttributesAll(MJAttributesAll),
    MJAttributesClass(MJAttributesClass),
    MJAttributesElement(MJAttributesElement),
}

from_child!(MJAttributesChild, MJAttributesAll);
from_child!(MJAttributesChild, MJAttributesClass);
from_child!(MJAttributesChild, MJAttributesElement);

impl MJAttributesChild {
    pub fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ALL => Ok(MJAttributesAll::parse(tokenizer)?.into()),
            MJ_CLASS => Ok(MJAttributesClass::parse(tokenizer)?.into()),
            _ => Ok(MJAttributesElement::parse(tag, tokenizer)?.into()),
        }
    }
}
