#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_raw::MJRawChild;
use crate::prelude::hash::Map;

pub const NAME: &str = "mj-navbar-link";

#[derive(Debug, Default)]
pub struct MJNavbarLink {
    attributes: Map<String, String>,
    children: Vec<MJRawChild>,
}
