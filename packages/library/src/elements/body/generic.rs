use crate::elements::body::comment::Comment;
use crate::elements::body::prelude::{BodyChild, BodyComponent};
use crate::elements::body::text::Text;

macro_rules! from_element {
    ($enum_name:ident) => {
        impl<E: BodyComponent> From<E> for $enum_name<E> {
            fn from(elt: E) -> Self {
                Self::Element(elt)
            }
        }
    };
}

#[derive(Debug)]
pub enum ComponentOrComment<E: BodyComponent> {
    Comment(Comment),
    Element(E),
}

impl<E: BodyComponent> ComponentOrComment<E> {
    pub fn comment(cmt: String) -> Self {
        Self::Comment(cmt.into())
    }
}

impl<E: BodyComponent> ComponentOrComment<E> {
    pub fn as_element(&self) -> Option<&E> {
        match self {
            Self::Element(elt) => Some(elt),
            _ => None,
        }
    }
}

impl<E: BodyComponent> BodyChild for ComponentOrComment<E> {
    fn inner_mut<'p>(&'p mut self) -> &'p mut (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
        }
    }

    fn inner<'p>(&'p self) -> &'p (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
        }
    }
}

from_element!(ComponentOrComment);

#[derive(Debug)]
pub enum ComponentOrTextOrComment<E: BodyComponent> {
    Comment(Comment),
    Element(E),
    Text(Text),
}

impl<E: BodyComponent> ComponentOrTextOrComment<E> {
    pub fn comment(cmt: String) -> Self {
        Self::Comment(cmt.into())
    }

    pub fn text(cmt: String) -> Self {
        Self::Text(cmt.into())
    }
}

impl<E: BodyComponent> BodyChild for ComponentOrTextOrComment<E> {
    fn inner_mut<'p>(&'p mut self) -> &'p mut (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
            Self::Text(elt) => elt,
        }
    }

    fn inner<'p>(&'p self) -> &'p (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

from_element!(ComponentOrTextOrComment);
