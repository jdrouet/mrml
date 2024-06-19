use crate::comment::Comment;
use crate::fragment::Fragment;
use crate::node::Node;
use crate::text::Text;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjRawChild {
    Comment(Comment),
    Fragment(Fragment<Self>),
    Node(Node<MjRawChild>),
    Text(Text),
}
