use crate::elements::head::mj_attributes::{MJAttributes, NAME as MJ_ATTRIBUTES};
use crate::elements::head::mj_breakpoint::{MJBreakpoint, NAME as MJ_BREAKPOINT};
use crate::elements::head::mj_font::{MJFont, NAME as MJ_FONT};
use crate::elements::head::mj_preview::{MJPreview, NAME as MJ_PREVIEW};
use crate::elements::head::mj_style::{MJStyle, NAME as MJ_STYLE};
use crate::elements::head::mj_title::{MJTitle, NAME as MJ_TITLE};
use crate::elements::head::prelude::HeadComponent;
use crate::parser::Error;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

macro_rules! as_enum {
    ($enum_name:ident, $struct_name:ident, $func_name:ident) => {
        impl $enum_name {
            pub fn $func_name(&self) -> Option<&$struct_name> {
                match self {
                    Self::$struct_name(value) => Some(value),
                    _ => None,
                }
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum MJHeadChild {
    MJAttributes(MJAttributes),
    MJBreakpoint(MJBreakpoint),
    MJFont(MJFont),
    MJPreview(MJPreview),
    MJStyle(MJStyle),
    MJTitle(MJTitle),
}

impl MJHeadChild {
    pub fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<MJHeadChild, Error> {
        Ok(match tag.as_str() {
            MJ_ATTRIBUTES => Self::MJAttributes(MJAttributes::parse(tokenizer)?),
            MJ_BREAKPOINT => Self::MJBreakpoint(MJBreakpoint::parse(tokenizer)?),
            MJ_FONT => Self::MJFont(MJFont::parse(tokenizer)?),
            MJ_PREVIEW => Self::MJPreview(MJPreview::parse(tokenizer)?),
            MJ_STYLE => Self::MJStyle(MJStyle::parse(tokenizer)?),
            MJ_TITLE => Self::MJTitle(MJTitle::parse(tokenizer)?),
            _ => return Err(Error::UnexpectedElement(tag.start())),
        })
    }

    pub fn as_head_component(&self) -> &dyn HeadComponent {
        match self {
            Self::MJAttributes(elt) => elt,
            Self::MJBreakpoint(elt) => elt,
            Self::MJFont(elt) => elt,
            Self::MJPreview(elt) => elt,
            Self::MJStyle(elt) => elt,
            Self::MJTitle(elt) => elt,
        }
    }
}

impl HeadComponent for MJHeadChild {
    fn update_header(&self, header: &mut Header) {
        self.as_head_component().update_header(header);
    }
}

as_enum!(MJHeadChild, MJPreview, as_mj_preview);
as_enum!(MJHeadChild, MJTitle, as_mj_title);
