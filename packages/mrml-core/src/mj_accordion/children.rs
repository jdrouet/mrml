use crate::comment::Comment;
#[cfg(feature = "fragment")]
use crate::fragment::Fragment;
use crate::mj_accordion_element::MjAccordionElement;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjAccordionChild {
    Comment(Comment),
    #[cfg(feature = "fragment")]
    Fragment(Fragment<Self>),
    MjAccordionElement(MjAccordionElement),
}
