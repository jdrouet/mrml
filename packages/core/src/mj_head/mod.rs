mod children;
mod parse;
mod print;

pub use children::MJHeadChild;

pub const NAME: &str = "mj-head";

#[derive(Debug, Default)]
pub struct MJHead {
    children: Vec<MJHeadChild>,
}
