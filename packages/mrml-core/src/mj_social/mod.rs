mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::prelude::hash::Map;

pub use children::MJSocialChild;

pub const NAME: &str = "mj-social";

#[derive(Debug, Default)]
pub struct MJSocial {
    attributes: Map<String, String>,
    children: Vec<MJSocialChild>,
}
