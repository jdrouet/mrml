mod children;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::collections::HashMap;

pub use children::MJCarouselChild;

pub const NAME: &str = "mj-carousel";

#[derive(Debug, Default)]
pub struct MJCarousel {
    attributes: HashMap<String, String>,
    children: Vec<MJCarouselChild>,
}
