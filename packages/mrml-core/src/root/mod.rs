use crate::comment::Comment;
use crate::mjml::Mjml;

#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "render")]
mod render;

#[derive(Debug)]
enum RootChild {
    Mjml(Mjml),
    Comment(Comment),
}

#[derive(Debug)]
pub(crate) struct Root(Vec<RootChild>);

impl AsRef<[RootChild]> for Root {
    fn as_ref(&self) -> &[RootChild] {
        self.0.as_ref()
    }
}

impl Root {
    pub(crate) fn into_mjml(self) -> Option<Mjml> {
        self.0.into_iter().find_map(|item| match item {
            RootChild::Mjml(inner) => Some(inner),
            _ => None,
        })
    }
}
