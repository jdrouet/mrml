#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

#[cfg(feature = "json")]
const NAME: &str = "comment";

#[derive(Debug, Default)]
#[cfg_attr(feature = "json", derive(mrml_json_macros::MrmlJsonComponent))]
#[cfg_attr(feature = "json", mrml_json(tag = "NAME"))]
pub struct Comment {
    pub children: String,
}

impl From<String> for Comment {
    fn from(children: String) -> Self {
        Self { children }
    }
}

impl From<&str> for Comment {
    fn from(value: &str) -> Self {
        Self::from(value.to_string())
    }
}
