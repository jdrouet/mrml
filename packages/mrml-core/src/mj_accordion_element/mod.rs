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
use std::collections::HashMap;

pub use children::MJAccordionElementChild;

pub const NAME: &str = "mj-accordion-element";

#[derive(Debug, Default)]
struct MJAccordionElementChildren {
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

impl MJAccordionElementChildren {
    fn is_empty(&self) -> bool {
        self.title.is_none() && self.text.is_none()
    }
}

#[derive(Debug, Default)]
pub struct MJAccordionElement {
    attributes: HashMap<String, String>,
    children: MJAccordionElementChildren,
}
