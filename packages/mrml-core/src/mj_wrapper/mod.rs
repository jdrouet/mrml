use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-wrapper";

#[derive(Debug, Default)]
pub struct MJWrapper {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
