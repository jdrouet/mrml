mod children;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::collections::HashMap;

pub use children::MJAccordionChild;

pub const NAME: &str = "mj-accordion";

#[derive(Debug, Default)]
pub struct MJAccordion {
    attributes: HashMap<String, String>,
    children: Vec<MJAccordionChild>,
}
