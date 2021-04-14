#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-breakpoint";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
struct MJBreakpointAttributes {
    value: String,
}

#[derive(Debug, Default)]
pub struct MJBreakpoint {
    attributes: MJBreakpointAttributes,
}

impl MJBreakpoint {
    pub fn value(&self) -> &str {
        &self.attributes.value
    }
}
