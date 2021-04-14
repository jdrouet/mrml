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
use std::collections::HashMap;

pub const NAME: &str = "mj-body";

#[derive(Debug, Default)]
pub struct MJBody {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
