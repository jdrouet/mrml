mod parser;
mod renderer;

use crate::util::size::Size;

pub const NAME: &str = "mj-breakpoint";

#[derive(Debug)]
pub struct MJBreakpoint {
    value: Size,
}
