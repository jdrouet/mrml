use xmlparser::StrSpan;

use super::MjButton;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjButton> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjButton, Error> {
        let (attributes, children) = self.parse_attributes_and_children()?;

        Ok(MjButton {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn success() {
        let raw = r#"<mj-button>
    <!-- Just a comment -->
    <b>foo</b>
    bar
</mj-button>"#;
        let _: MjButton = MrmlCursor::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
