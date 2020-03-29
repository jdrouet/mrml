mod renderer;

#[derive(Clone, Debug)]
pub struct Text {
    content: String,
}

impl From<String> for Text {
    fn from(content: String) -> Self {
        Self { content }
    }
}
