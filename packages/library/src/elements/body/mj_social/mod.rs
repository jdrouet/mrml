mod parser;
mod renderer;

use crate::elements::body::generic::ComponentOrComment;
use crate::elements::body::mj_social_element::MJSocialElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-social";

pub type MJSocialChild = ComponentOrComment<MJSocialElement>;

#[derive(Debug)]
pub struct MJSocial {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJSocialChild>,
}

fn build_children_attributes(attrs: &Attributes) -> Attributes {
    let mut result = Attributes::default();
    result.maybe_set("padding", attrs.get("inner-padding"));
    result.maybe_set("border-radius", attrs.get("border-radius"));
    result.maybe_set("color", attrs.get("color"));
    result.maybe_set("font-family", attrs.get("font-family"));
    result.maybe_set("font-size", attrs.get("font-size"));
    result.maybe_set("font-weight", attrs.get("font-weight"));
    result.maybe_set("font-style", attrs.get("font-style"));
    result.maybe_set("icon-size", attrs.get("icon-size"));
    result.maybe_set("icon-height", attrs.get("icon-height"));
    result.maybe_set("icon-padding", attrs.get("icon-padding"));
    result.maybe_set("text-padding", attrs.get("text-padding"));
    result.maybe_set("line-height", attrs.get("line-height"));
    result.maybe_set("text-decoration", attrs.get("text-decoration"));
    result
}
