mod parser;
mod renderer;

use super::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-text";

#[derive(Clone, Debug)]
pub struct MJText {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}
