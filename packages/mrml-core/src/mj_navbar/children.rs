use crate::comment::Comment;
use crate::mj_navbar_link::MjNavbarLink;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjNavbarChild {
    Comment(Comment),
    MjNavbarLink(MjNavbarLink),
}
