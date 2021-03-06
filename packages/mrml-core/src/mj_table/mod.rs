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

pub const NAME: &str = "mj-table";

#[derive(Debug, Default)]
pub struct MJTable {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
