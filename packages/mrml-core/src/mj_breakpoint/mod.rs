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

impl MJBreakpointAttributes {
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
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
