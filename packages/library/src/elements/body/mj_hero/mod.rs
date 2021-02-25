mod parser;
mod renderer;

use crate::elements::body::mj_body::children::MJBodyChild;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-hero";

#[derive(Debug)]
pub struct MJHero {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJBodyChild>,
}
