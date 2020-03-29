mod parser;
mod renderer;

pub const NAME: &str = "mj-preview";

#[derive(Clone, Debug, Default)]
pub struct MJPreview {
    pub content: String,
}
