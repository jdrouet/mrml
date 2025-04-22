mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

use std::marker::PhantomData;

pub use crate::mj_selector::MjSelector;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-html-attributes";

pub struct MjHtmlAttributesTag;

impl StaticTag for MjHtmlAttributesTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjHtmlAttributes = Component<PhantomData<MjHtmlAttributesTag>, (), Vec<MjSelector>>;

#[cfg(feature = "render")]
impl MjHtmlAttributes {
    pub(crate) fn mj_selector_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.children.iter().flat_map(|child| {
            child.children.iter().map(|c| {
                (
                    child.attributes.path.as_str(),
                    c.attributes.name.as_str(),
                    c.children.as_str(),
                )
            })
        })
    }
}

impl MjHtmlAttributes {
    pub fn children(&self) -> &Vec<MjSelector> {
        &self.children
    }
}
