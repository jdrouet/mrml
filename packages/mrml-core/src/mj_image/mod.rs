use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-image";

#[derive(Debug, Default)]
pub struct MJImage {
    attributes: HashMap<String, String>,
}
