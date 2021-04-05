mod children;
mod parse;
mod print;
mod render;

pub use children::MJNavbarLinkChild;
use std::collections::HashMap;

pub const NAME: &str = "mj-navbar-link";

#[derive(Debug, Default)]
pub struct MJNavbarLink {
    attributes: HashMap<String, String>,
    children: Vec<MJNavbarLinkChild>,
}
