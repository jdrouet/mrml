mod children;
mod parse;
mod print;

pub use children::MJAttributesChild;

pub const NAME: &str = "mj-attributes";

#[derive(Debug, Default)]
pub struct MJAttributes {
    children: Vec<MJAttributesChild>,
}
