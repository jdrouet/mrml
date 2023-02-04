//! Module containing the `mj-accordion` element as defined in [the documentation](https://documentation.mjml.io/#mj-accordion).
//!
//! ```rust
//! let template = include_str!("../../resources/compare/success/mj-accordion.mjml");
//! let root = mrml::parse(template).expect("parse template");
//! let opts = mrml::prelude::render::Options::default();
//! match root.render(&opts) {
//!     Ok(content) => println!("{content}"),
//!     Err(_) => println!("couldn't render mjml template"),
//! };
//! ```

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

pub use children::MjAccordionChild;

pub const NAME: &str = "mj-accordion";

#[derive(Debug, Default)]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseComponent))]
#[cfg_attr(feature = "parse", mrml_parse(child_text = false))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAccordion {
    pub attributes: Map<String, String>,
    pub children: Vec<MjAccordionChild>,
}
