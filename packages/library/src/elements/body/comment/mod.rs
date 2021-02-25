mod renderer;

#[derive(Debug)]
pub struct Comment {
    content: String,
}

impl From<String> for Comment {
    fn from(content: String) -> Self {
        Self { content }
    }
}
