use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-text";

#[derive(Debug, Default)]
pub struct MJText {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
