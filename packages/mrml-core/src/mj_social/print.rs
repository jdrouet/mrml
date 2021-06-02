use super::{MJSocial, MJSocialChild};
use crate::print_attrs_children;

impl Print for MJSocialChild {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        match self {
            Self::Comment(elt) => elt.print(pretty, level, indent_size),
            Self::MJSocialElement(elt) => elt.print(pretty, level, indent_size),
        }
    }
}

print_attrs_children!(MJSocial, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_social::MJSocial::default();
        assert_eq!("<mj-social></mj-social>", item.dense_print());
    }
}
