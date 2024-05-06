use crate::mj_attributes_all::MjAttributesAll;
use crate::mj_attributes_class::MjAttributesClass;
use crate::mj_attributes_element::MjAttributesElement;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
#[cfg_attr(feature = "render", derive(enum_as_inner::EnumAsInner))]
pub enum MjAttributesChild {
    MjAttributesAll(MjAttributesAll),
    MjAttributesClass(MjAttributesClass),
    MjAttributesElement(MjAttributesElement),
}
