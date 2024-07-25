mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

pub use children::MjHeadChild;

use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-head";

pub struct MjHeadTag;

impl StaticTag for MjHeadTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjHead = Component<PhantomData<MjHeadTag>, (), Vec<MjHeadChild>>;

#[cfg(feature = "render")]
impl MjHead {
    pub fn breakpoint(&self) -> Option<&crate::mj_breakpoint::MjBreakpoint> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_breakpoint().into_iter().chain(
                    item.as_mj_include()
                        .into_iter()
                        .filter(|item| item.attributes.kind.is_mjml())
                        .flat_map(|inner| {
                            inner
                                .children
                                .iter()
                                .filter_map(|child| child.as_mj_breakpoint())
                        }),
                )
            })
            .last()
    }

    pub fn preview(&self) -> Option<&crate::mj_preview::MjPreview> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_preview().into_iter().chain(
                    item.as_mj_include()
                        .into_iter()
                        .filter(|item| item.attributes.kind.is_mjml())
                        .flat_map(|inner| {
                            inner
                                .children
                                .iter()
                                .filter_map(|child| child.as_mj_preview())
                        }),
                )
            })
            .last()
    }

    pub fn title(&self) -> Option<&crate::mj_title::MjTitle> {
        self.children
            .iter()
            .flat_map(|item| {
                item.as_mj_title().into_iter().chain(
                    item.as_mj_include()
                        .into_iter()
                        .filter(|item| item.attributes.kind.is_mjml())
                        .flat_map(|inner| {
                            inner
                                .children
                                .iter()
                                .filter_map(|child| child.as_mj_title())
                        }),
                )
            })
            .last()
    }

    pub fn children(&self) -> &Vec<MjHeadChild> {
        &self.children
    }
}
