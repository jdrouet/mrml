mod parser;
mod renderer;

use crate::util::size::Size;

#[derive(Clone, Debug)]
pub struct MJBreakpoint {
    value: Option<Size>,
}
