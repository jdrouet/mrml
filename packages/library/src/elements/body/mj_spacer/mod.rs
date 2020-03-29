mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-spacer";

#[derive(Clone, Debug)]
pub struct MJSpacer {
    attributes: Attributes,
    context: Option<Context>,
}
