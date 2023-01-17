mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use crate::mj_breakpoint::MJBreakpoint;
use crate::mj_preview::MJPreview;
use crate::mj_title::MJTitle;
pub use children::MJHeadChild;

pub const NAME: &str = "mj-head";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
pub struct MJHead {
    children: Vec<MJHeadChild>,
}

impl MJHead {
    pub fn breakpoint(&self) -> Option<&MJBreakpoint> {
        self.children
            .iter()
            .find_map(|item| item.as_mj_breakpoint())
    }

    pub fn preview(&self) -> Option<&MJPreview> {
        self.children.iter().find_map(|item| item.as_mj_preview())
    }

    pub fn title(&self) -> Option<&MJTitle> {
        self.children.iter().find_map(|item| item.as_mj_title())
    }

    pub fn children(&self) -> &Vec<MJHeadChild> {
        &self.children
    }
}
