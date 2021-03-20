use crate::comment::Comment;
use crate::from_child;
use crate::mj_button::MJButton;
use crate::node::Node;
use crate::prelude::print::Print;
use crate::text::Text;

#[derive(Debug)]
pub enum MJBodyChild {
    Comment(Comment),
    MJButton(MJButton),
    Node(Node),
    Text(Text),
}

from_child!(MJBodyChild, Comment);
from_child!(MJBodyChild, MJButton);
from_child!(MJBodyChild, Node);
from_child!(MJBodyChild, Text);

impl MJBodyChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJButton(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}
