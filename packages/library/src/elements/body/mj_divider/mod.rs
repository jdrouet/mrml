mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-divider";

#[derive(Clone, Debug)]
pub struct MJDivider {
    attributes: Attributes,
    context: Option<Context>,
}
