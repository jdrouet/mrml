#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_raw::MJRawChild;
use std::collections::HashMap;

pub const NAME: &str = "mj-accordion-text";

#[derive(Debug, Default)]
pub struct MJAccordionText {
    attributes: HashMap<String, String>,
    children: Vec<MJRawChild>,
}
