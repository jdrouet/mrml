#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::prelude::hash::Map;

pub const NAME: &str = "mj-carousel-image";

#[derive(Debug, Default)]
pub struct MJCarouselImage {
    attributes: Map<String, String>,
}
