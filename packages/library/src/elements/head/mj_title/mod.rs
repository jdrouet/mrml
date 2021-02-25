mod parser;
mod renderer;

pub const NAME: &str = "mj-title";

#[derive(Debug)]
pub struct MJTitle {
    content: String,
}

impl MJTitle {
    pub fn get_content(&self) -> String {
        self.content.clone()
    }
}
