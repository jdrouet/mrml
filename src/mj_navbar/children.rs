use crate::comment::Comment;
use crate::from_child;
use crate::mj_navbar_link::MJNavbarLink;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MJNavbarChild {
    Comment(Comment),
    MJNavbarLink(MJNavbarLink),
}

from_child!(MJNavbarChild, Comment);
from_child!(MJNavbarChild, MJNavbarLink);
