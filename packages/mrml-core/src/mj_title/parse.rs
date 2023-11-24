use xmlparser::StrSpan;

use super::MjTitle;
use crate::prelude::parser::{ElementParser, Error, MrmlCursor};

impl<'a> ElementParser<'a, MjTitle> for MrmlCursor<'a> {
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
    use crate::mj_title::MjTitle;
    use crate::prelude::parser::MrmlCursor;

    #[test]
    fn success() {
        let _: MjTitle =
            MrmlCursor::new("<mj-title>Hello World!</mj-title>", Default::default())
                .parse_root()
                .unwrap();
    }
}
