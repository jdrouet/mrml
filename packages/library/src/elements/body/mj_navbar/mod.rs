mod parser;
mod renderer;

use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-navbar";

#[derive(Clone, Debug)]
pub struct MJNavbar {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    id: String,
}

impl MJNavbar {
    fn get_children_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.maybe_set("navbar-base-url", self.get_attribute("base-url"));
        attrs
    }
}
