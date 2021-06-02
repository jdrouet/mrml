use crate::comment::Comment;
use crate::from_child;
use crate::text::Text;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJNavbarLinkChild {
    Comment(Comment),
    Text(Text),
}

from_child!(MJNavbarLinkChild, Comment);
from_child!(MJNavbarLinkChild, Text);
