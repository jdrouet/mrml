use crate::elements::head::mj_attributes_all::MJAttributesAll;
use crate::elements::head::mj_attributes_class::MJAttributesClass;
use crate::elements::head::mj_attributes_element::MJAttributesElement;
use crate::util::header::DefaultAttributes;

pub const NAME: &str = "mj-attributes";

#[derive(Clone, Debug)]
pub enum MJAttributesChild {
    All(MJAttributesAll),
    Class(MJAttributesClass),
    Element(MJAttributesElement),
}

impl MJAttributesChild {
    pub fn update_attributes(&self, result: &mut DefaultAttributes) {
        match self {
            Self::All(elt) => elt.update_attributes(result),
            Self::Class(elt) => elt.update_attributes(result),
            Self::Element(elt) => elt.update_attributes(result),
        }
    }
}

impl From<MJAttributesAll> for MJAttributesChild {
    fn from(elt: MJAttributesAll) -> Self {
        Self::All(elt)
    }
}

impl From<MJAttributesClass> for MJAttributesChild {
    fn from(elt: MJAttributesClass) -> Self {
        Self::Class(elt)
    }
}

impl From<MJAttributesElement> for MJAttributesChild {
    fn from(elt: MJAttributesElement) -> Self {
        Self::Element(elt)
    }
}
