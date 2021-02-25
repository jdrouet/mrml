mod parser;
mod renderer;

use crate::elements::body::generic::ComponentOrComment;
use crate::elements::body::mj_accordion_element::MJAccordionElement;
use crate::util::attributes::Attributes;
use crate::util::context::Context;

pub const NAME: &str = "mj-accordion";

pub type MJAccordionChild = ComponentOrComment<MJAccordionElement>;

#[derive(Clone, Debug)]
pub struct MJAccordion {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJAccordionChild>,
}
