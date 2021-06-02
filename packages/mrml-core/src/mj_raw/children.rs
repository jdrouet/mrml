use crate::comment::Comment;
use crate::from_child;
use crate::node::Node;
use crate::text::Text;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJRawChild {
    Comment(Comment),
    Node(Node<MJRawChild>),
    Text(Text),
}

from_child!(MJRawChild, Comment);
from_child!(MJRawChild, Text);

impl From<Node<MJRawChild>> for MJRawChild {
    fn from(value: Node<MJRawChild>) -> Self {
        Self::Node(value)
    }
}
