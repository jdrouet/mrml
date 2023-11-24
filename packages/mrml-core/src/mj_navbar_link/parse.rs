use xmlparser::StrSpan;

use super::MjNavbarLink;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjNavbarLink> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjNavbarLink, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjNavbarLink {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjNavbarLink;
    use crate::prelude::parser::MrmlCursor;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            #[test]
            fn $title() {
                let _: MjNavbarLink = MrmlCursor::new($template, Default::default())
                    .parse_root()
                    .unwrap();
            }
        };
    }

    assert_success!(should_handle_empty_children, "<mj-navbar-link />");

    assert_success!(
        should_handle_comments,
        "<mj-navbar-link><!-- comment --></mj-navbar-link>"
    );

    assert_success!(
        should_work_with_text,
        "<mj-navbar-link>Hello</mj-navbar-link>"
    );

    assert_success!(
        should_work_with_other_element,
        "<mj-navbar-link><span /></mj-navbar-link>"
    );
}
