#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "fragment";

#[derive(Debug, Clone)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct Fragment<T> {
    pub children: Vec<T>,
}

impl<T> Default for Fragment<T> {
    fn default() -> Self {
        Self {
            children: Default::default(),
        }
    }
}

impl<T> From<Vec<T>> for Fragment<T> {
    fn from(children: Vec<T>) -> Self {
        Self { children }
    }
}
