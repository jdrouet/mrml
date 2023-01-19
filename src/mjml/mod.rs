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
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub struct MJMLChildren {
    head: Option<MJHead>,
    body: Option<MJBody>,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MJML {
    pub attributes: Map<String, String>,
    pub children: MJMLChildren,
}

impl MJML {
    pub fn body(&self) -> Option<&MJBody> {
        self.children.body.as_ref()
    }

    pub fn head(&self) -> Option<&MJHead> {
        self.children.head.as_ref()
    }
}
