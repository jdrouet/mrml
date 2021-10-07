mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_accordion_text::MJAccordionText;
use crate::mj_accordion_title::MJAccordionTitle;
use crate::prelude::hash::Map;

pub use children::MJAccordionElementChild;

pub const NAME: &str = "mj-accordion-element";

#[derive(Debug, Default)]
struct MJAccordionElementChildren {
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

#[derive(Debug, Default)]
pub struct MJAccordionElement {
    attributes: Map<String, String>,
    children: MJAccordionElementChildren,
}
