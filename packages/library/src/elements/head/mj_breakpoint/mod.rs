mod parser;
mod renderer;

use crate::util::size::Size;

pub const NAME: &str = "mj-breakpoint";

#[derive(Clone, Debug)]
pub struct MJBreakpoint {
    value: Option<Size>,
}
