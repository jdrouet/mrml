#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::collections::HashMap;

pub const NAME: &str = "mj-carousel-image";

#[derive(Debug, Default)]
pub struct MJCarouselImage {
    attributes: HashMap<String, String>,
}
