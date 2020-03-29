mod parser;
mod renderer;

use crate::elements::body::prelude::*;
use crate::elements::body::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-social";

#[derive(Clone, Debug)]
pub struct MJSocial {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJSocial {
    fn get_children_attributes(&self) -> Attributes {
        let mut attrs = Attributes::default();
        attrs.maybe_set("padding", self.get_attribute("inner-padding"));
        attrs.maybe_set("border-radius", self.get_attribute("border-radius"));
        attrs.maybe_set("color", self.get_attribute("color"));
        attrs.maybe_set("font-family", self.get_attribute("font-family"));
        attrs.maybe_set("font-size", self.get_attribute("font-size"));
        attrs.maybe_set("font-weight", self.get_attribute("font-weight"));
        attrs.maybe_set("font-style", self.get_attribute("font-style"));
        attrs.maybe_set("icon-size", self.get_attribute("icon-size"));
        attrs.maybe_set("icon-height", self.get_attribute("icon-height"));
        attrs.maybe_set("icon-padding", self.get_attribute("icon-padding"));
        attrs.maybe_set("text-padding", self.get_attribute("text-padding"));
        attrs.maybe_set("line-height", self.get_attribute("line-height"));
        attrs.maybe_set("text-decoration", self.get_attribute("text-decoration"));
        attrs
    }
}
