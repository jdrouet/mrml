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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Component<Tag, Attributes, Children> {
    pub tag: Tag,
    pub attributes: Attributes,
    pub children: Children,
}

impl<T: Default, A: Default, C: Default> Default for Component<T, A, C> {
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

pub type AttributeMap = hash::Map<String, Option<String>>;

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "json", serde(untagged))]
pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

impl<T> Default for OneOrMany<T> {
    fn default() -> Self {
        OneOrMany::Many(Vec::default())
    }
}

impl<T> OneOrMany<T> {
    pub fn iter(&self) -> OneOrManyIter<'_, T> {
        match self {
            Self::One(item) => OneOrManyIter::One(std::iter::once(&item)),
            Self::Many(list) => OneOrManyIter::Many(list.iter()),
        }
    }
}

pub enum OneOrManyIter<'a, T> {
    One(std::iter::Once<&'a T>),
    Many(std::slice::Iter<'a, T>),
}

impl<'a, T> Iterator for OneOrManyIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::One(inner) => inner.next(),
            Self::Many(inner) => inner.next(),
        }
    }
}
