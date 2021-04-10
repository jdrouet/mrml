mod children;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_accordion_text::MJAccordionText;
use crate::mj_accordion_title::MJAccordionTitle;
use std::collections::HashMap;

pub use children::MJAccordionElementChild;

pub const NAME: &str = "mj-accordion-element";

#[derive(Debug, Default)]
pub struct MJAccordionElement {
    attributes: HashMap<String, String>,
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}
