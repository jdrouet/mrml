#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub type Node<T> = crate::prelude::Component<String, crate::prelude::AttributeMap, Vec<T>>;

impl<N: Into<String>, T> From<N> for Node<T> {
    fn from(tag: N) -> Self {
        Self {
            tag: tag.into(),
            attributes: crate::prelude::AttributeMap::new(),
            children: Vec::new(),
        }
    }
}
