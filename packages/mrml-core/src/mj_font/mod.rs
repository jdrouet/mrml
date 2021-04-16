#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-font";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
struct MJFontAttributes {
    name: String,
    href: String,
}

impl MJFontAttributes {
    fn is_empty(&self) -> bool {
        self.name.is_empty() && self.href.is_empty()
    }
}

#[derive(Debug, Default)]
pub struct MJFont {
    attributes: MJFontAttributes,
}

impl MJFont {
    pub fn name(&self) -> &str {
        &self.attributes.name
    }

    pub fn href(&self) -> &str {
        &self.attributes.href
    }
}
