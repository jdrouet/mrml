mod parser;

use crate::elements::body::generic::ComponentOrTextOrComment;
use crate::elements::body::mj_body::children::MJBodyChild;
use crate::elements::body::node::Node;

pub type RawElement = ComponentOrTextOrComment<Node>;

impl Into<MJBodyChild> for RawElement {
    fn into(self) -> MJBodyChild {
        match self {
            Self::Comment(value) => MJBodyChild::Comment(value),
            Self::Element(value) => MJBodyChild::Node(value),
            Self::Text(value) => MJBodyChild::Text(value),
        }
    }
}
