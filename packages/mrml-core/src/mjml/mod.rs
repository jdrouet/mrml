#![allow(dead_code)]

use std::marker::PhantomData;

use crate::mj_body::MjBody;
use crate::mj_head::MjHead;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mjml";

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjmlAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub owa: Option<String>,
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub lang: Option<String>,
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub dir: Option<String>,
}

impl MjmlAttributes {
    fn lang(&self) -> &str {
        self.lang.as_deref().unwrap_or("und")
    }

    fn dir(&self) -> &str {
        self.dir.as_deref().unwrap_or("auto")
    }
}

#[derive(Clone, Debug, Default)]
pub struct MjmlChildren {
    pub head: Option<MjHead>,
    pub body: Option<MjBody>,
}

pub struct MjmlTag;

impl StaticTag for MjmlTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

/// Representation of the `mjml` and its attributes and children defined
/// in the [mjml documentation](https://documentation.mjml.io/#mjml).
pub type Mjml = Component<PhantomData<MjmlTag>, MjmlAttributes, MjmlChildren>;

impl Mjml {
    pub fn body(&self) -> Option<&MjBody> {
        self.children.body.as_ref()
    }

    pub fn head(&self) -> Option<&MjHead> {
        self.children.head.as_ref()
    }
}
