use crate::comment::Comment;
use crate::node::Node;
use crate::text::Text;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MjRawChild {
    Comment(Comment),
    Node(Node<MjRawChild>),
    Text(Text),
}
