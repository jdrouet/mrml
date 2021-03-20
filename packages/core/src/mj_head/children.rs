use crate::comment::Comment;
use crate::from_child;
use crate::mj_title::MJTitle;
use crate::prelude::print::Print;

#[derive(Debug)]
pub enum MJHeadChild {
    Comment(Comment),
    MJTitle(MJTitle),
}

from_child!(MJHeadChild, Comment);
from_child!(MJHeadChild, MJTitle);

impl MJHeadChild {
    pub fn as_print<'p>(&'p self) -> &'p (dyn Print + 'p) {
        match self {
            Self::Comment(elt) => elt,
            Self::MJTitle(elt) => elt,
        }
    }
}
