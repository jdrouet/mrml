mod parse;
mod print;
mod render;

use crate::text::Text;
use std::collections::HashMap;

pub const NAME: &str = "mj-accordion-title";

#[derive(Debug, Default)]
pub struct MJAccordionTitle {
    attributes: HashMap<String, String>,
    children: Vec<Text>,
}
