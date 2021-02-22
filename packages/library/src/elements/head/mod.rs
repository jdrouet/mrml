use crate::elements::error::Error;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

pub mod mj_attributes;
pub mod mj_attributes_all;
pub mod mj_attributes_class;
pub mod mj_attributes_element;
pub mod mj_breakpoint;
pub mod mj_font;
pub mod mj_head;
pub mod mj_preview;
pub mod mj_style;
pub mod mj_title;
pub mod prelude;

#[derive(Clone, Debug)]
pub enum HeadElement {
    MJAttributes(mj_attributes::MJAttributes),
    MJBreakpoint(mj_breakpoint::MJBreakpoint),
    MJFont(mj_font::MJFont),
    MJPreview(mj_preview::MJPreview),
    MJStyle(mj_style::MJStyle),
    MJTitle(mj_title::MJTitle),
}

macro_rules! apply_fn {
    ($root:expr, $func:ident($($args:tt)*)) => {
        match $root {
            HeadElement::MJAttributes(item) => item.$func($($args)*),
            HeadElement::MJBreakpoint(item) => item.$func($($args)*),
            HeadElement::MJFont(item) => item.$func($($args)*),
            HeadElement::MJPreview(item) => item.$func($($args)*),
            HeadElement::MJStyle(item) => item.$func($($args)*),
            HeadElement::MJTitle(item) => item.$func($($args)*),
        }
    };
}

impl HeadElement {
    pub fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<HeadElement, Error> {
        let res = match tag.as_str() {
            mj_attributes::NAME => {
                HeadElement::MJAttributes(mj_attributes::MJAttributes::parse(tokenizer)?)
            }
            mj_breakpoint::NAME => {
                HeadElement::MJBreakpoint(mj_breakpoint::MJBreakpoint::parse(tokenizer)?)
            }
            mj_font::NAME => HeadElement::MJFont(mj_font::MJFont::parse(tokenizer)?),
            mj_preview::NAME => HeadElement::MJPreview(mj_preview::MJPreview::parse(tokenizer)?),
            mj_style::NAME => HeadElement::MJStyle(mj_style::MJStyle::parse(tokenizer)?),
            mj_title::NAME => HeadElement::MJTitle(mj_title::MJTitle::parse(tokenizer)?),
            _ => return Err(Error::UnexpectedElement(tag.to_string())),
        };
        Ok(res)
    }
}

impl prelude::HeadComponent for HeadElement {
    fn update_header(&self, header: &mut Header) {
        apply_fn!(self, update_header(header));
    }
}

impl HeadElement {
    pub fn as_mj_preview(&self) -> Option<&mj_preview::MJPreview> {
        match self {
            Self::MJPreview(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_mj_title(&self) -> Option<&mj_title::MJTitle> {
        match self {
            Self::MJTitle(value) => Some(value),
            _ => None,
        }
    }
}
