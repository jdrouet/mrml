mod parser;
mod renderer;

use crate::elements::body::mj_accordion_text::MJAccordionText;
use crate::elements::body::mj_accordion_title::MJAccordionTitle;
use crate::elements::body::prelude::*;
use crate::util::attributes::*;
use crate::util::context::Context;

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

#[derive(Clone, Debug)]
pub struct MJAccordionElement {
    attributes: Attributes,
    context: Option<Context>,
    title: Option<MJAccordionTitle>,
    text: Option<MJAccordionText>,
}

impl MJAccordionElement {
    fn get_children_attributes(&self) -> Attributes {
        let mut result = Attributes::default();
        for key in CHILDREN_ATTR.iter() {
            if let Some(value) = self.get_attribute(key) {
                result.set(key, value);
            }
        }
        result
    }
}
