mod parser;
mod renderer;

use crate::elements::body::BodyElement;
use crate::util::attributes::Attributes;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct NodeElement {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    tag: String,
}

#[derive(Clone, Debug)]
pub enum RawElement {
    Comment(String),
    Node(NodeElement),
    Text(String),
}
