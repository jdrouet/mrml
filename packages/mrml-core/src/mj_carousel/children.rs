use crate::comment::Comment;
#[cfg(feature = "fragment")]
use crate::fragment::Fragment;
use crate::mj_carousel_image::MjCarouselImage;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
pub enum MjCarouselChild {
    Comment(Comment),
    #[cfg(feature = "fragment")]
    Fragment(Fragment<Self>),
    MjCarouselImage(MjCarouselImage),
}

impl MjCarouselChild {
    pub(crate) fn as_mj_carousel_image(&self) -> Option<&MjCarouselImage> {
        match self {
            Self::MjCarouselImage(inner) => Some(inner),
            _ => None,
        }
    }
}
