mod children;
#[cfg(feature = "json")]
mod json;
mod network;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub use children::MJSocialElementChild;
use std::collections::HashMap;

pub const NAME: &str = "mj-social-element";

#[derive(Debug, Default)]
pub struct MJSocialElement {
    attributes: HashMap<String, String>,
    children: Vec<MJSocialElementChild>,
}
