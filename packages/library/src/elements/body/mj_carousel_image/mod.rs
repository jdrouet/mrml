mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

#[derive(Clone, Debug)]
pub struct MJCarouselImage {
    attributes: Attributes,
    carousel_id: String,
    context: Option<Context>,
    content: Option<String>,
}
