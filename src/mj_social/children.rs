use crate::comment::Comment;
use crate::from_child;
use crate::mj_social_element::MJSocialElement;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJSocialChild {
    Comment(Comment),
    MJSocialElement(MJSocialElement),
}

from_child!(MJSocialChild, Comment);
from_child!(MJSocialChild, MJSocialElement);
