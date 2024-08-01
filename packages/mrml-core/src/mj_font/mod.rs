use std::marker::PhantomData;

use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-font";

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjFontAttributes {
    pub name: String,
    pub href: String,
}

pub struct MjFontTag;

impl StaticTag for MjFontTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjFont = Component<PhantomData<MjFontTag>, MjFontAttributes, ()>;

#[cfg(all(test, any(feature = "render", feature = "print")))]
impl MjFont {
    pub(crate) fn build<N: Into<String>, H: Into<String>>(name: N, href: H) -> Self {
        Self::new(
            MjFontAttributes {
                name: name.into(),
                href: href.into(),
            },
            (),
        )
    }
}

impl MjFont {
    pub fn name(&self) -> &str {
        &self.attributes.name
    }

    pub fn href(&self) -> &str {
        &self.attributes.href
    }
}
