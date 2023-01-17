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
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
struct MJAccordionElementChildren {
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
pub struct MJAccordionElement {
    attributes: Map<String, String>,
    children: MJAccordionElementChildren,
}
