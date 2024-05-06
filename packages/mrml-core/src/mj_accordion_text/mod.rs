#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_raw::MjRawChild;
use crate::prelude::hash::Map;

pub const NAME: &str = "mj-accordion-text";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAccordionText {
    pub attributes: Map<String, String>,
    pub children: Vec<MjRawChild>,
}
