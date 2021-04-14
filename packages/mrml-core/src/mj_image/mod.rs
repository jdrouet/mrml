use std::collections::HashMap;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-image";

#[derive(Debug, Default)]
pub struct MJImage {
    attributes: HashMap<String, String>,
}
