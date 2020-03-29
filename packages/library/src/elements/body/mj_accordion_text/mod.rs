mod parser;
mod renderer;

use crate::elements::body::raw::RawElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-accordion-text";

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
