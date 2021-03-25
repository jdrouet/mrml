mod children;
mod parse;
mod print;
mod render;

pub use children::MJBodyChild;
use std::collections::HashMap;

pub const NAME: &str = "mj-body";

#[derive(Debug, Default)]
pub struct MJBody {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
