mod parser;
mod renderer;

use crate::elements::head::mj_attributes_all::MJAttributesAll;
use crate::elements::head::mj_attributes_class::MJAttributesClass;
use crate::elements::head::mj_attributes_element::MJAttributesElement;

pub const NAME: &str = "mj-attributes";

#[derive(Clone, Debug)]
pub enum MjAttributesChild {
    All(MJAttributesAll),
    Class(MJAttributesClass),
    Element(MJAttributesElement),
}

impl From<MJAttributesAll> for MjAttributesChild {
    fn from(elt: MJAttributesAll) -> Self {
        Self::All(elt)
    }
}

impl From<MJAttributesClass> for MjAttributesChild {
    fn from(elt: MJAttributesClass) -> Self {
        Self::Class(elt)
    }
}

impl From<MJAttributesElement> for MjAttributesChild {
    fn from(elt: MJAttributesElement) -> Self {
        Self::Element(elt)
    }
}

#[derive(Clone, Debug, Default)]
pub struct MJAttributes {
    children: Vec<MjAttributesChild>,
}
