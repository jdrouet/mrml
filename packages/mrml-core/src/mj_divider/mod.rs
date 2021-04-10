use std::collections::HashMap;

#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-divider";

#[derive(Debug, Default)]
pub struct MJDivider {
    attributes: HashMap<String, String>,
}
