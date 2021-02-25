mod parser;
mod renderer;

use crate::elements::body::generic::ComponentOrComment;
use crate::elements::body::mj_navbar_link::MJNavbarLink;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-navbar";

pub type MJNavbarChild = ComponentOrComment<MJNavbarLink>;

#[derive(Clone, Debug)]
pub struct MJNavbar {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJNavbarChild>,
    id: String,
}

fn build_children_attributes(attrs: &Attributes) -> Attributes {
    let mut result = Attributes::default();
    result.maybe_set("navbar-base-url", attrs.get("base-url"));
    result
}
