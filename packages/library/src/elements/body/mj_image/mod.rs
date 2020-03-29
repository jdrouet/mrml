mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-image";

#[derive(Clone, Debug)]
pub struct MJImage {
    attributes: Attributes,
    context: Option<Context>,
}
