mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

use std::marker::PhantomData;

pub use children::MjAttributesChild;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-attributes";

pub struct MjAttributesTag;

impl StaticTag for MjAttributesTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjAttributes = Component<PhantomData<MjAttributesTag>, (), Vec<MjAttributesChild>>;

#[cfg(feature = "render")]
impl MjAttributes {
    pub(crate) fn mj_attributes_all_iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.children
            .iter()
            .filter_map(|child| child.as_mj_attributes_all())
            .flat_map(|child| {
                child
                    .attributes
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.as_str()))
            })
    }

    pub(crate) fn mj_attributes_class_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.children
            .iter()
            .filter_map(|child| child.as_mj_attributes_class())
            .flat_map(|child| {
                child
                    .attributes
                    .iter()
                    .map(move |(k, v)| (child.name.as_str(), k.as_str(), v.as_str()))
            })
    }

    pub(crate) fn mj_attributes_element_iter(&self) -> impl Iterator<Item = (&str, &str, &str)> {
        self.children
            .iter()
            .filter_map(|child| child.as_mj_attributes_element())
            .flat_map(|child| {
                child
                    .attributes
                    .iter()
                    .map(move |(k, v)| (child.name.as_str(), k.as_str(), v.as_str()))
            })
    }
}

impl MjAttributes {
    pub fn children(&self) -> &Vec<MjAttributesChild> {
        &self.children
    }
}
