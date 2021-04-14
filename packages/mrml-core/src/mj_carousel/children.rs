use crate::comment::Comment;
use crate::mj_carousel_image::{MJCarouselImage, NAME as MJ_CAROUSEL_IMAGE};
#[cfg(feature = "parse")]
use crate::prelude::parse::{Error as ParserError, Parsable};
#[cfg(feature = "print")]
use crate::prelude::print::Print;
#[cfg(feature = "render")]
use crate::prelude::render::{Header, Render, Renderable};
use crate::{as_child, from_child};
use std::cell::RefCell;
use std::rc::Rc;
#[cfg(feature = "parse")]
use xmlparser::{StrSpan, Tokenizer};

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

#[cfg(feature = "print")]
impl Print for MJCarouselChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJCarouselImage(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

#[cfg(feature = "parse")]
impl Parsable for MJCarouselChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, ParserError> {
        match tag.as_str() {
            MJ_CAROUSEL_IMAGE => Ok(MJCarouselImage::parse(tag, tokenizer)?.into()),
            _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(feature = "render")]
impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJCarouselChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MJCarouselImage(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}
