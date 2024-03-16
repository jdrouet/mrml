#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub const NAME: &str = "mj-font";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintAttributes))]
pub struct MjFontAttributes {
    name: String,
    href: String,
}

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjFont {
    pub attributes: MjFontAttributes,
}

#[cfg(all(test, feature = "render"))]
impl MjFont {
    pub(crate) fn new<N: Into<String>, H: Into<String>>(name: N, href: H) -> Self {
        Self {
            attributes: MjFontAttributes {
                name: name.into(),
                href: href.into(),
            },
        }
    }
}

impl MjFont {
    pub fn name(&self) -> &str {
        &self.attributes.name
    }

    pub fn href(&self) -> &str {
        &self.attributes.href
    }
}
