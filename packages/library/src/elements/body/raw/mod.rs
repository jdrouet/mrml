mod parser;
mod renderer;

use crate::elements::body::comment::Comment;
use crate::elements::body::node::Node;
use crate::elements::body::text::Text;
use crate::elements::body::BodyElement;

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

impl Into<BodyElement> for RawElement {
    fn into(self) -> BodyElement {
        match self {
            Self::Comment(value) => BodyElement::Comment(value),
            Self::Node(value) => BodyElement::Node(value),
            Self::Text(value) => BodyElement::Text(value),
        }
    }
}
