use crate::comment::Comment;
use crate::mj_social_element::MjSocialElement;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjSocialChild {
    Comment(Comment),
    MjSocialElement(MjSocialElement),
}
