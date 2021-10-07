use crate::mj_body::MJBody;
use crate::mj_head::MJHead;
use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mjml";

#[derive(Debug, Default)]
struct MJMLChildren {
    head: Option<MJHead>,
    body: Option<MJBody>,
}

#[derive(Debug, Default)]
pub struct MJML {
    attributes: Map<String, String>,
    children: MJMLChildren,
}

impl MJML {
    pub fn body(&self) -> Option<&MJBody> {
        self.children.body.as_ref()
    }

    pub fn head(&self) -> Option<&MJHead> {
        self.children.head.as_ref()
    }
}
