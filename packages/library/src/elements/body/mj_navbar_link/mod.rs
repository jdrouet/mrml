mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-navbar-link";

#[derive(Clone, Debug)]
pub struct MJNavbarLink {
    attributes: Attributes,
    context: Option<Context>,
    content: Option<String>,
}
