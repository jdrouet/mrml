use crate::comment::Comment;
use crate::mj_social_element::MjSocialElement;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjSocialChild {
    Comment(Comment),
    MjSocialElement(MjSocialElement),
}
