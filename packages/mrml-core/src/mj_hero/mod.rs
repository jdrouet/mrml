use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-hero";

#[derive(Debug, Default)]
pub struct MJHero {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
