mod children;
mod parse;
mod print;
mod render;

pub use children::MJRawChild;

pub const NAME: &str = "mj-raw";

#[derive(Debug, Default)]
pub struct MJRaw {
    children: Vec<MJRawChild>,
}
