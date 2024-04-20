use crate::comment::Comment;
use crate::mjml::Mjml;

#[cfg(feature = "parse")]
pub mod parse;
#[cfg(feature = "render")]
pub mod render;

#[derive(Debug)]
/// Representation of the `mjml` and its attributes and children defined
/// in the [mjml documentation](https://documentation.mjml.io/#mjml).
pub enum RootChild {
    Mjml(Mjml),
    Comment(Comment),
}

#[derive(Debug)]
pub struct Root(Vec<RootChild>);

impl AsRef<[RootChild]> for Root {
    fn as_ref(&self) -> &[RootChild] {
        self.0.as_ref()
    }
}

impl Root {
    pub fn as_mjml(&self) -> Option<&Mjml> {
        self.0.iter().find_map(|item| match item {
            RootChild::Mjml(inner) => Some(inner),
            _ => None,
        })
    }

    pub fn into_mjml(self) -> Option<Mjml> {
        self.0.into_iter().find_map(|item| match item {
            RootChild::Mjml(inner) => Some(inner),
            _ => None,
        })
    }
}
