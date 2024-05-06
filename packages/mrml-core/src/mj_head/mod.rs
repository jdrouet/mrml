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

pub const NAME: &str = "mj-head";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjHead {
    pub children: Vec<MjHeadChild>,
}

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
