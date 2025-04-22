use crate::prelude::{Component, StaticTag};
use std::marker::PhantomData;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-html-attribute";

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjHtmlAttributeAttributes {
    pub name: String,
}
pub struct MjHtmlAttributeTag;

impl StaticTag for MjHtmlAttributeTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjHtmlAttribute =
    Component<PhantomData<MjHtmlAttributeTag>, MjHtmlAttributeAttributes, String>;

impl MjHtmlAttribute {
    pub fn name(&self) -> &str {
        self.attributes.name.as_str()
    }

    pub fn children(&self) -> &str {
        &self.children
    }
}

impl From<String> for MjHtmlAttribute {
    fn from(children: String) -> Self {
        Self::new(MjHtmlAttributeAttributes::default(), children)
    }
}

impl From<&str> for MjHtmlAttribute {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
