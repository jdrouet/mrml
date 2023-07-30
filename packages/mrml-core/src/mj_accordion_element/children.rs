use crate::comment::Comment;
use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjAccordionElementChild {
    Comment(Comment),
    MjAccordionText(MjAccordionText),
    MjAccordionTitle(MjAccordionTitle),
}
