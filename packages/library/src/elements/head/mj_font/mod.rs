mod parser;
mod renderer;

pub const NAME: &str = "mj-font";

#[derive(Debug)]
pub struct MJFont {
    name: String,
    href: String,
}
