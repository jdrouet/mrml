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

pub const NAME: &str = "mj-group";

#[derive(Debug, Default)]
pub struct MJGroup {
    attributes: Map<String, String>,
    children: Vec<MJBodyChild>,
}
