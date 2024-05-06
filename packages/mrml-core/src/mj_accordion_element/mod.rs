#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;
use crate::prelude::hash::Map;

pub const NAME: &str = "mj-accordion-element";

#[derive(Debug, Default)]
pub struct MjAccordionElementChildren {
    pub title: Option<MjAccordionTitle>,
    pub text: Option<MjAccordionText>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAccordionElement {
    pub attributes: Map<String, String>,
    pub children: MjAccordionElementChildren,
}
