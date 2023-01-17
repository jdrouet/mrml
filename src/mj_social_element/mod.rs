#[cfg(feature = "json")]
mod json;
#[cfg(feature = "render")]
mod network;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_raw::MJRawChild;
use crate::prelude::hash::Map;

pub const NAME: &str = "mj-social-element";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
pub struct MJSocialElement {
    pub attributes: Map<String, String>,
    pub children: Vec<MJRawChild>,
}
