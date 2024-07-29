use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub struct CommentTag;

impl StaticTag for CommentTag {
    fn static_tag() -> &'static str {
        "comment"
    }
}

pub type Comment = Component<PhantomData<CommentTag>, (), String>;

impl<V: Into<String>> From<V> for Comment {
    fn from(value: V) -> Self {
        Self::new((), value.into())
    }
}
