use crate::mj_attributes_all::MJAttributesAll;
use crate::mj_attributes_class::MJAttributesClass;
use crate::mj_attributes_element::MJAttributesElement;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MJAttributesChild {
    MJAttributesAll(MJAttributesAll),
    MJAttributesClass(MJAttributesClass),
    MJAttributesElement(MJAttributesElement),
}
