use crate::prelude::hash::Map;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub type Node<T> = crate::prelude::Component<String, Map<String, String>, Vec<T>>;

impl<N: Into<String>, T> From<N> for Node<T> {
    fn from(tag: N) -> Self {
        Self {
            tag: tag.into(),
            attributes: Map::new(),
            children: Vec::new(),
        }
    }
}
