use crate::mj_attributes_all::MjAttributesAll;
use crate::mj_attributes_class::MjAttributesClass;
use crate::mj_attributes_element::MjAttributesElement;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjAttributesChild {
    MjAttributesAll(MjAttributesAll),
    MjAttributesClass(MjAttributesClass),
    MjAttributesElement(MjAttributesElement),
}
