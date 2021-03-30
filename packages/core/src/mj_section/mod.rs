use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-section";

#[derive(Debug, Default)]
pub struct MJSection {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
