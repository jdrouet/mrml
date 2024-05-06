#![allow(dead_code)]

use crate::mj_body::MjBody;
use crate::mj_head::MjHead;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mjml";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct MjmlAttributes {
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub owa: Option<String>,
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub lang: Option<String>,
    #[cfg_attr(feature = "json", serde(skip_serializing_if = "Option::is_none"))]
    pub dir: Option<String>,
}

#[derive(Debug, Default)]
pub struct MjmlChildren {
    pub head: Option<MjHead>,
    pub body: Option<MjBody>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
/// Representation of the `mjml` and its attributes and children defined
/// in the [mjml documentation](https://documentation.mjml.io/#mjml).
pub struct Mjml {
    pub attributes: MjmlAttributes,
    pub children: MjmlChildren,
}

impl Mjml {
    pub fn body(&self) -> Option<&MjBody> {
        self.children.body.as_ref()
    }

    pub fn head(&self) -> Option<&MjHead> {
        self.children.head.as_ref()
    }
}
