use xmlparser::StrSpan;

use super::MjPreview;
use crate::prelude::parser::{ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjPreview> for MrmlParser<'a> {
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
    use crate::{mj_preview::MjPreview, prelude::parser::MrmlParser};

    #[test]
    fn success() {
        let _: MjPreview = MrmlParser::new(
            r#"<mj-preview>Hello World!</mj-preview>"#,
            Default::default(),
        )
        .parse_root()
        .unwrap();
    }
}
