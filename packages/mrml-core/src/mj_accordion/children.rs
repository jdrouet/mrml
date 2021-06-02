use crate::comment::Comment;
use crate::mj_accordion_element::MJAccordionElement;
use crate::{as_child, from_child};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJAccordionChild {
    Comment(Comment),
    MJAccordionElement(MJAccordionElement),
}

as_child!(MJAccordionChild, MJAccordionElement, as_element);
from_child!(MJAccordionChild, Comment);
from_child!(MJAccordionChild, MJAccordionElement);
