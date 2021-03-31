use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

mod parse;
mod print;
mod render;

pub const NAME: &str = "mj-hero";

#[derive(Debug, Default)]
pub struct MJHero {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
