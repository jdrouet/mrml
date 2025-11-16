use std::marker::PhantomData;

use crate::comment::Comment;
use crate::prelude::{Component, StaticTag};
use crate::text::Text;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-preview";

pub struct MjPreviewTag;

impl StaticTag for MjPreviewTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjPreviewChild {
    Comment(Comment),
    Text(Text),
}

pub type MjPreview = Component<PhantomData<MjPreviewTag>, (), Vec<MjPreviewChild>>;

impl MjPreview {
    pub fn content(&self) -> String {
        self.children
            .iter()
            .filter_map(|item| match item {
                MjPreviewChild::Text(inner) => Some(inner.inner_str()),
                _ => None,
            })
            .collect::<String>()
    }
}

impl From<String> for MjPreview {
    fn from(children: String) -> Self {
        Self::new((), vec![MjPreviewChild::Text(Text::from(children))])
    }
}

impl From<&str> for MjPreview {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
