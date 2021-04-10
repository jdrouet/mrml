use std::collections::HashMap;

#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-spacer";

#[derive(Debug, Default)]
pub struct MJSpacer {
    attributes: HashMap<String, String>,
}
