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

impl<V: Into<String>> From<V> for Comment {
    fn from(value: V) -> Self {
        Self {
            children: value.into(),
        }
    }
}
