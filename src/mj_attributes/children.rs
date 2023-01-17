use crate::mj_attributes_all::MJAttributesAll;
use crate::mj_attributes_class::MJAttributesClass;
use crate::mj_attributes_element::MJAttributesElement;
use crate::{as_child, from_child};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
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
