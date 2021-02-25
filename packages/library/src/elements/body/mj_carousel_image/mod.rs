mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-carousel-image";

#[derive(Debug)]
pub struct MJCarouselImage {
    attributes: Attributes,
    carousel_id: String,
    context: Option<Context>,
    content: Option<String>,
}
