use std::marker::PhantomData;

#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "parse")]
pub mod parser;
#[cfg(feature = "print")]
pub mod print;
#[cfg(feature = "render")]
pub mod render;

pub mod hash;

pub trait StaticTag {
    fn static_tag() -> &'static str;
}

#[derive(Clone, Debug)]
pub struct Component<Tag, Attributes, Children> {
    pub tag: Tag,
    pub attributes: Attributes,
    pub children: Children,
}

impl<
        T: Clone + std::fmt::Debug + Default,
        A: Clone + std::fmt::Debug + Default,
        C: Clone + std::fmt::Debug + Default,
    > Default for Component<T, A, C>
{
    fn default() -> Self {
        Self {
            tag: T::default(),
            attributes: A::default(),
            children: C::default(),
        }
    }
}

impl<T, A, C> Component<PhantomData<T>, A, C> {
    #[inline]
    pub fn new(attributes: A, children: C) -> Self {
        Self {
            tag: PhantomData::<T>,
            attributes,
            children,
        }
    }
}

// see https://developer.mozilla.org/en-US/docs/Glossary/Void_element
#[cfg(any(feature = "parse", feature = "print", feature = "render"))]
pub(crate) fn is_void_element(tag: &str) -> bool {
    matches!(
        tag,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "link"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}
