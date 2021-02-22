mod parser;
mod renderer;

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

fn build_children_attributes(attrs: &Attributes) -> Attributes {
    let mut result = Attributes::default();
    result.maybe_set("navbar-base-url", attrs.get("base-url"));
    result
}
