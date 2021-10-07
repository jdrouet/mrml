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
pub use children::MJNavbarLinkChild;

pub const NAME: &str = "mj-navbar-link";

#[derive(Debug, Default)]
pub struct MJNavbarLink {
    attributes: Map<String, String>,
    children: Vec<MJNavbarLinkChild>,
}
