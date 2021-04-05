use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-divider";

#[derive(Debug, Default)]
pub struct MJDivider {
    attributes: HashMap<String, String>,
}
