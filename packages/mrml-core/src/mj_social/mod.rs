mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::collections::HashMap;

pub use children::MJSocialChild;

pub const NAME: &str = "mj-social";

#[derive(Debug, Default)]
pub struct MJSocial {
    attributes: HashMap<String, String>,
    children: Vec<MJSocialChild>,
}
