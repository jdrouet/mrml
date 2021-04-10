mod children;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub use children::MJRawChild;

pub const NAME: &str = "mj-raw";

#[derive(Debug, Default)]
pub struct MJRaw {
    children: Vec<MJRawChild>,
}
