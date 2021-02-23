mod parser;
mod renderer;

use crate::elements::body::mj_body::children::MJBodyChild;
use crate::util::context::Context;

pub const NAME: &str = "mj-raw";

#[derive(Clone, Debug)]
pub struct MJRaw {
    context: Option<Context>,
    children: Vec<MJBodyChild>,
}
