mod parser;
mod renderer;

use crate::elements::body::mj_accordion_text::MJAccordionText;
use crate::elements::body::mj_accordion_title::MJAccordionTitle;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-accordion-element";

const CHILDREN_ATTR: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

fn build_children_attributes(attrs: &Attributes) -> Attributes {
    let mut result = Attributes::default();
    for key in CHILDREN_ATTR.iter() {
        if let Some(value) = attrs.get(key) {
            result.set(key, value);
        }
    }
    result
}

#[derive(Clone, Debug)]
pub struct MJAccordionElement {
    attributes: Attributes,
    context: Option<Context>,
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

impl MJAccordionElement {
    fn get_children_attributes(&self) -> Attributes {
        build_children_attributes(&self.attributes)
    }
}
