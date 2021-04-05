mod children;
mod parse;
mod print;
mod render;

use std::collections::HashMap;

pub use children::MJNavbarChild;

pub const NAME: &str = "mj-navbar";

#[derive(Debug, Default)]
pub struct MJNavbar {
    attributes: HashMap<String, String>,
    children: Vec<MJNavbarChild>,
}
