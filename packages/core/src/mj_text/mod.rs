use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-text";

#[derive(Debug, Default)]
pub struct MJText {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
