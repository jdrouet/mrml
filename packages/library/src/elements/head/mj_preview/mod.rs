mod parser;
mod renderer;

pub const NAME: &str = "mj-preview";

#[derive(Debug, Default)]
pub struct MJPreview {
    pub content: String,
}
