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
    use crate::{mj_title::MjTitle, prelude::parser::MrmlParser};

    #[test]
    fn success() {
        let _: MjTitle = MrmlParser::new("<mj-title>Hello World!</mj-title>", Default::default())
            .parse_root()
            .unwrap();
    }
}
