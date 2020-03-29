mod parser;
mod renderer;

pub const NAME: &str = "mj-font";

#[derive(Clone, Debug)]
pub struct MJFont {
    name: String,
    href: String,
}
