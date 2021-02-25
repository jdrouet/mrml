mod parser;
mod renderer;

use crate::elements::body::generic::ComponentOrComment;
use crate::elements::body::mj_carousel_image::MJCarouselImage;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-carousel";

pub type MJCarouselChild = ComponentOrComment<MJCarouselImage>;

#[derive(Debug)]
pub struct MJCarousel {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJCarouselChild>,
    id: String,
}

fn build_children_attributes(id: &str, attrs: &Attributes) -> Attributes {
    Attributes::default()
        .add("carousel-id", id)
        .maybe_add("border-radius", attrs.get("border-radius"))
        .maybe_add("tb-border", attrs.get("tb-border"))
        .maybe_add("tb-border-radius", attrs.get("tb-border-radius"))
}
