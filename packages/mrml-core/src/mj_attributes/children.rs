use crate::mj_attributes_all::MJAttributesAll;
use crate::mj_attributes_all::NAME as MJ_ALL;
use crate::mj_attributes_class::MJAttributesClass;
use crate::mj_attributes_class::NAME as MJ_CLASS;
use crate::mj_attributes_element::MJAttributesElement;
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "print")]
use crate::prelude::print::Print;
use crate::{as_child, from_child};
#[cfg(feature = "parse")]
use xmlparser::{StrSpan, Tokenizer};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJAttributesChild {
    MJAttributesAll(MJAttributesAll),
    MJAttributesClass(MJAttributesClass),
    MJAttributesElement(MJAttributesElement),
}

as_child!(MJAttributesChild, MJAttributesAll, as_mj_all);
from_child!(MJAttributesChild, MJAttributesAll);
as_child!(MJAttributesChild, MJAttributesClass, as_mj_class);
from_child!(MJAttributesChild, MJAttributesClass);
as_child!(MJAttributesChild, MJAttributesElement, as_element);
from_child!(MJAttributesChild, MJAttributesElement);

#[cfg(feature = "parse")]
impl Parsable for MJAttributesChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_ALL => Ok(MJAttributesAll::parse(tag, tokenizer)?.into()),
            MJ_CLASS => Ok(MJAttributesClass::parse(tag, tokenizer)?.into()),
            _ => Ok(MJAttributesElement::parse(tag, tokenizer)?.into()),
        }
    }
}

#[cfg(feature = "print")]
impl MJAttributesChild {
    fn as_print(&self) -> &dyn Print {
        match self {
            Self::MJAttributesAll(elt) => elt,
            Self::MJAttributesClass(elt) => elt,
            Self::MJAttributesElement(elt) => elt,
        }
    }
}

#[cfg(feature = "print")]
impl Print for MJAttributesChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        self.as_print().print(pretty, level, indent_size)
    }
}
