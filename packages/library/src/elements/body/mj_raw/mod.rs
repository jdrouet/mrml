mod parser;
mod renderer;

use crate::elements::body::BodyElement;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct MJRaw {
    context: Option<Context>,
    children: Vec<BodyElement>,
}
