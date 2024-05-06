use crate::comment::Comment;
use crate::mj_accordion_element::MjAccordionElement;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjAccordionChild {
    Comment(Comment),
    MjAccordionElement(MjAccordionElement),
}
