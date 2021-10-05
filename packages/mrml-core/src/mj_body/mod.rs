mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub use children::MJBodyChild;
use crate::prelude::hash::Map;

pub const NAME: &str = "mj-body";

#[derive(Debug, Default)]
pub struct MJBody {
    attributes: Map<String, String>,
    children: Vec<MJBodyChild>,
}
