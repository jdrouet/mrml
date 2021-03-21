mod parse;
mod print;

pub const NAME: &str = "mj-font";

#[derive(Debug, Default)]
pub struct MJFont {
    name: String,
    href: String,
}
