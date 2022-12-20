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

pub use children::MJNavbarChild;

pub const NAME: &str = "mj-navbar";

#[derive(Debug, Default)]
pub struct MJNavbar {
    attributes: Map<String, String>,
    children: Vec<MJNavbarChild>,
}
