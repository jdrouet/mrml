pub mod children;
mod parser;
mod renderer;

pub const NAME: &str = "mj-attributes";

#[derive(Clone, Debug, Default)]
pub struct MJAttributes {
    children: Vec<children::MJAttributesChild>,
}
