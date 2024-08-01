use crate::comment::Comment;
use crate::mj_carousel_image::MjCarouselImage;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjCarouselChild {
    Comment(Comment),
    MjCarouselImage(MjCarouselImage),
}
