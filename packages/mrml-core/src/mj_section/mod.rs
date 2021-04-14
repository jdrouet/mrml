use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[cfg(feature = "render")]
pub use render::WithMJSectionBackground;

pub const NAME: &str = "mj-section";

#[derive(Debug, Default)]
pub struct MJSection {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
