#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::text::Text;
use crate::prelude::hash::Map;

pub const NAME: &str = "mj-accordion-title";

#[derive(Debug, Default)]
pub struct MJAccordionTitle {
    attributes: Map<String, String>,
    children: Vec<Text>,
}
