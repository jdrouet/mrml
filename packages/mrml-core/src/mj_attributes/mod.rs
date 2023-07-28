mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub use children::MjAttributesChild;

pub const NAME: &str = "mj-attributes";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct MjAttributes {
    children: Vec<MjAttributesChild>,
}

impl MjAttributes {
    pub fn children(&self) -> &Vec<MjAttributesChild> {
        &self.children
    }
}
