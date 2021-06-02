use crate::comment::Comment;
use crate::from_child;
use crate::text::Text;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJSocialElementChild {
    Comment(Comment),
    Text(Text),
}

from_child!(MJSocialElementChild, Comment);
from_child!(MJSocialElementChild, Text);
