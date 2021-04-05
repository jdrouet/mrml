mod parse;
mod print;
mod render;

use std::collections::HashMap;

pub const NAME: &str = "mj-carousel-image";

#[derive(Debug, Default)]
pub struct MJCarouselImage {
    attributes: HashMap<String, String>,
}
