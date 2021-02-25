use crate::elements::body::comment::Comment;
use crate::elements::body::prelude::{BodyComponent, BodyComponentChildIterator};
use crate::elements::body::text::Text;
use crate::elements::prelude::Component;
use crate::elements::Error;
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;

macro_rules! propagate_trait {
    ($enum_name:ident) => {
        impl<E: BodyComponent> $enum_name<E> {
            pub fn as_element(&self) -> Option<&E> {
                match self {
                    Self::Element(elt) => Some(elt),
                    _ => None,
                }
            }
        }

        impl<E: BodyComponent> From<E> for $enum_name<E> {
            fn from(elt: E) -> Self {
                Self::Element(elt)
            }
        }

        impl<E: BodyComponent> Component for $enum_name<E> {
            fn render(&self, header: &Header) -> Result<String, Error> {
                self.inner().render(header)
            }

            fn context(&self) -> Option<&Context> {
                self.inner().context()
            }

            fn set_context(&mut self, ctx: Context) {
                self.inner_mut().set_context(ctx)
            }
        }

        impl<E: BodyComponent> BodyComponent for $enum_name<E> {
            fn is_raw(&self) -> bool {
                self.inner().is_raw()
            }

            fn attributes(&self) -> Option<&Attributes> {
                self.inner().attributes()
            }

            fn get_children_len(&self) -> usize {
                self.inner().get_children_len()
            }

            fn get_children<'p>(&'p self) -> BodyComponentChildIterator<'p> {
                self.inner().get_children()
            }

            fn get_current_width(&self) -> Option<Size> {
                self.inner().get_current_width()
            }
        }
    };
}

#[derive(Clone, Debug)]
pub enum ComponentOrComment<E: BodyComponent> {
    Comment(Comment),
    Element(E),
}

impl<E: BodyComponent> ComponentOrComment<E> {
    pub fn comment(cmt: String) -> Self {
        Self::Comment(cmt.into())
    }

    pub fn inner_mut<'p>(&'p mut self) -> &'p mut (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
        }
    }

    pub fn inner<'p>(&'p self) -> &'p (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
        }
    }
}

propagate_trait!(ComponentOrComment);

#[derive(Clone, Debug)]
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

    pub fn inner_mut<'p>(&'p mut self) -> &'p mut (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
            Self::Text(elt) => elt,
        }
    }

    pub fn inner<'p>(&'p self) -> &'p (dyn BodyComponent + 'p) {
        match self {
            Self::Element(elt) => elt,
            Self::Comment(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

propagate_trait!(ComponentOrTextOrComment);
