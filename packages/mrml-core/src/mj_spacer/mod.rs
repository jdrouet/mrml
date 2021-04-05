use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-spacer";

#[derive(Debug, Default)]
pub struct MJSpacer {
    attributes: HashMap<String, String>,
}
