mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-accordion-title";

#[derive(Clone, Debug)]
pub struct MJAccordionTitle {
    attributes: Attributes,
    context: Option<Context>,
    content: String,
}

impl MJAccordionTitle {
    pub fn new(attributes: Attributes) -> Self {
        MJAccordionTitle {
            attributes,
            context: None,
            content: "".into(),
        }
    }
}
