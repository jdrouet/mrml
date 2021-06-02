use crate::comment::Comment;
use crate::mj_carousel_image::MJCarouselImage;
use crate::{as_child, from_child};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum MJCarouselChild {
    Comment(Comment),
    MJCarouselImage(MJCarouselImage),
}

as_child!(MJCarouselChild, MJCarouselImage, as_image);
from_child!(MJCarouselChild, Comment);
from_child!(MJCarouselChild, MJCarouselImage);
