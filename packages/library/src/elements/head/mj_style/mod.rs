mod parser;
mod renderer;

pub const NAME: &str = "mj-style";

#[derive(Debug)]
pub struct MJStyle {
    content: String,
    inline: bool,
}
