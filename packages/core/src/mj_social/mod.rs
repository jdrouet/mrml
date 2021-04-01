mod children;
mod parse;
mod print;
mod render;

use std::collections::HashMap;

pub use children::MJSocialChild;

pub const NAME: &str = "mj-social";

#[derive(Debug, Default)]
pub struct MJSocial {
    attributes: HashMap<String, String>,
    children: Vec<MJSocialChild>,
}
