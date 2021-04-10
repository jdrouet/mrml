#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-font";

#[derive(Debug, Default)]
pub struct MJFont {
    name: String,
    href: String,
}

impl MJFont {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn href(&self) -> &str {
        &self.href
    }
}
