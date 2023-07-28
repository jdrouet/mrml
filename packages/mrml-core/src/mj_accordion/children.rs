use crate::comment::Comment;
use crate::mj_accordion_element::MjAccordionElement;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjAccordionChild {
    Comment(Comment),
    MjAccordionElement(MjAccordionElement),
}
