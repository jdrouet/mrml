use crate::comment::Comment;
use crate::conditional_comment::ConditionalComment;
use crate::node::Node;
use crate::text::Text;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjRawChild {
    ConditionalComment(ConditionalComment),
    Comment(Comment),
    Node(Node<MjRawChild>),
    Text(Text),
}
