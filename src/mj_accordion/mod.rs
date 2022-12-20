mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::prelude::hash::Map;

pub use children::MJAccordionChild;

pub const NAME: &str = "mj-accordion";

#[derive(Debug, Default)]
pub struct MJAccordion {
    attributes: Map<String, String>,
    children: Vec<MJAccordionChild>,
}
