mod children;
mod parse;
mod print;

pub use children::MJHeadChild;

pub const NAME: &str = "mj-head";

#[derive(Debug, Default)]
pub struct MJHead {
    children: Vec<MJHeadChild>,
}

impl MJHead {
    pub fn children(&self) -> &Vec<MJHeadChild> {
        &self.children
    }
}
