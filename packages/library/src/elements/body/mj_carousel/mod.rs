mod parser;
mod renderer;

use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct MJCarousel {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    id: String,
}

impl MJCarousel {
    fn get_children_attributes(&self) -> Attributes {
        Attributes::default()
            .add("carousel-id", self.id.as_str())
            .maybe_add("border-radius", self.get_attribute("border-radius"))
            .maybe_add("tb-border", self.get_attribute("tb-border"))
            .maybe_add("tb-border-radius", self.get_attribute("tb-border-radius"))
    }
}
