mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub use children::MjHeadChild;

use crate::mj_breakpoint::MjBreakpoint;
use crate::mj_preview::MjPreview;
use crate::mj_title::MjTitle;

pub const NAME: &str = "mj-head";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjHead {
    pub children: Vec<MjHeadChild>,
}

impl MjHead {
    pub fn breakpoint(&self) -> Option<&MjBreakpoint> {
        self.children
            .iter()
            .filter_map(|item| {
                if let Some(title) = item.as_mj_breakpoint() {
                    Some(title)
                } else if let Some(include) = item.as_mj_include() {
                    include
                        .children
                        .iter()
                        .filter_map(|child| child.as_mj_breakpoint())
                        .next()
                } else {
                    None
                }
            })
            .next()
    }

    pub fn preview(&self) -> Option<&MjPreview> {
        self.children
            .iter()
            .filter_map(|item| {
                if let Some(title) = item.as_mj_preview() {
                    Some(title)
                } else if let Some(include) = item.as_mj_include() {
                    include
                        .children
                        .iter()
                        .filter_map(|child| child.as_mj_preview())
                        .next()
                } else {
                    None
                }
            })
            .next()
    }

    pub fn title(&self) -> Option<&MjTitle> {
        self.children
            .iter()
            .filter_map(|item| {
                if let Some(title) = item.as_mj_title() {
                    Some(title)
                } else if let Some(include) = item.as_mj_include() {
                    include
                        .children
                        .iter()
                        .filter_map(|child| child.as_mj_title())
                        .next()
                } else {
                    None
                }
            })
            .next()
    }

    pub fn children(&self) -> &Vec<MjHeadChild> {
        &self.children
    }
}
