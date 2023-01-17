use crate::comment::Comment;
use crate::mj_accordion_element::MJAccordionElement;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MJAccordionChild {
    Comment(Comment),
    MJAccordionElement(MJAccordionElement),
}
