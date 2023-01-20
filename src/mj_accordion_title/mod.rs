#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::prelude::hash::Map;
use crate::text::Text;

pub const NAME: &str = "mj-accordion-title";

#[derive(Debug, Default)]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseComponent))]
#[cfg_attr(
    feature = "parse",
    mrml_parse(child_comment = false, child_element = false)
)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MJAccordionTitle {
    pub attributes: Map<String, String>,
    pub children: Vec<Text>,
}
