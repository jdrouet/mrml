mod children;
#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;

pub use children::MJAttributesChild;

pub const NAME: &str = "mj-attributes";

#[derive(Debug, Default)]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintComponent))]
#[cfg_attr(feature = "print", mrml_print(tag = "NAME"))]
pub struct MJAttributes {
    children: Vec<MJAttributesChild>,
}

impl MJAttributes {
    pub fn children(&self) -> &Vec<MJAttributesChild> {
        &self.children
    }
}
