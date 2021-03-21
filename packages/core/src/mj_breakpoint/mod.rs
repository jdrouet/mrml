mod parse;
mod print;

pub const NAME: &str = "mj-breakpoint";

#[derive(Debug, Default)]
pub struct MJBreakpoint {
    value: String,
}
