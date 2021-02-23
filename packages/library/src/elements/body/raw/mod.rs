mod parser;
mod renderer;

use crate::elements::body::comment::Comment;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::node::Node;
use crate::elements::body::text::Text;

#[derive(Clone, Debug)]
pub enum RawElement {
    Comment(Comment),
    Node(Node),
    Text(Text),
}

impl RawElement {
    pub fn comment(value: String) -> Self {
        Self::Comment(Comment::from(value))
    }

    pub fn text(value: String) -> Self {
        Self::Text(Text::from(value))
    }
}

impl Into<MJBodyChild> for RawElement {
    fn into(self) -> MJBodyChild {
        match self {
            Self::Comment(value) => MJBodyChild::Comment(value),
            Self::Node(value) => MJBodyChild::Node(value),
            Self::Text(value) => MJBodyChild::Text(value),
        }
    }
}
