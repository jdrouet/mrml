mod children;
mod parse;
mod print;
mod render;

use std::collections::HashMap;

pub use children::MJCarouselChild;

pub const NAME: &str = "mj-carousel";

#[derive(Debug, Default)]
pub struct MJCarousel {
    attributes: HashMap<String, String>,
    children: Vec<MJCarouselChild>,
}
