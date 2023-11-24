use xmlparser::StrSpan;

use super::MjPreview;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjPreview> for MrmlCursor<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjPreview, Error> {
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjPreview::default());
        }

        let text = self.next_text()?.map(|inner| inner.text.to_string());

        self.assert_element_close()?;

        Ok(MjPreview {
            children: text.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_preview::MjPreview;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn should_parse() {
        let _: MjPreview = MrmlCursor::new(
            r#"<mj-preview>Hello World!</mj-preview>"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
    }

    #[test]
    fn should_parse_without_children() {
        let _: MjPreview = MrmlCursor::new(r#"<mj-preview />"#, Default::default())
            .parse_root()
            .unwrap();
    }
}
