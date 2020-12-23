mod parser;
mod renderer;

#[derive(Clone, Debug)]
pub struct MJStyle {
    content: String,
    inline: bool,
}
