use xmlparser::StrSpan;

use super::MjTitle;
use crate::prelude::parser::{ElementParser, Error, MrmlParser};

impl<'a> ElementParser<'a, MjTitle> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjTitle, Error> {
        let ending = self.assert_element_end()?;
        if ending.empty {
            return Ok(MjTitle::default());
        }

        let text = self.next_text()?.map(|inner| inner.text.to_string());

        self.assert_element_close()?;

        Ok(MjTitle {
            children: text.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        let res = crate::mjml::Mjml::parse(
            r#"<mjml><mj-head><mj-title>Hello World!</mj-title></mj-head></mjml>"#,
        );
        assert!(res.is_ok());
    }
}
