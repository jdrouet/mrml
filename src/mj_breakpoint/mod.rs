#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-breakpoint";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintAttributes))]
struct MJBreakpointAttributes {
    width: String,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
pub struct MJBreakpoint {
    attributes: MJBreakpointAttributes,
}

impl MJBreakpoint {
    pub fn value(&self) -> &str {
        &self.attributes.width
    }
}
