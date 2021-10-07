mod children;
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

use crate::prelude::hash::Map;
pub use children::MJSocialElementChild;

pub const NAME: &str = "mj-social-element";

#[derive(Debug, Default)]
pub struct MJSocialElement {
    attributes: Map<String, String>,
    children: Vec<MJSocialElementChild>,
}
