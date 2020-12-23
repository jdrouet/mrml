mod parser;
mod renderer;

#[derive(Clone, Debug)]
pub struct MJTitle {
    content: String,
}

impl MJTitle {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
