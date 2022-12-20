use crate::mj_body::MJBodyChild;
use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-button";

#[derive(Debug, Default)]
pub struct MJButton {
    attributes: Map<String, String>,
    children: Vec<MJBodyChild>,
}
