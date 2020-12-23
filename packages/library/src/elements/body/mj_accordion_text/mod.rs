mod parser;
mod renderer;

use crate::elements::body::raw::RawElement;
use crate::util::attributes::*;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct MJAccordionText {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJAccordionText {
    pub fn new(attributes: Attributes) -> Self {
        MJAccordionText {
            attributes,
            context: None,
            children: vec![],
        }
    }
}
