mod parser;
mod renderer;

use crate::elements::body::BodyElement;
use crate::util::attributes::Attributes;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}
