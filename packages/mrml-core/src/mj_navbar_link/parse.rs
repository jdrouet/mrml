use xmlparser::StrSpan;

use super::MjNavbarLink;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjNavbarLink> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<MjNavbarLink, Error> {
        let (attributes, children) = self.parse_attributes_and_children(cursor)?;

        Ok(MjNavbarLink {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MjNavbarLink;

    macro_rules! assert_success {
        ($title:ident, $template:expr) => {
            crate::should_sync_parse!($title, MjNavbarLink, $template);
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
