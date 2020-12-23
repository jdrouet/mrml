mod parser;
mod renderer;

use crate::elements::body::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct MJAccordion {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}
