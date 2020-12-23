mod parser;
mod renderer;

use super::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct MJText {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}
