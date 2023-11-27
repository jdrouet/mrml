use xmlparser::StrSpan;

use super::MjTitle;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseElement};

impl ParseElement<MjTitle> for MrmlParser {
    fn parse<'a>(&self, cursor: &mut MrmlCursor<'a>, _: StrSpan<'a>) -> Result<MjTitle, Error> {
        let ending = cursor.assert_element_end()?;
        if ending.empty {
            return Ok(MjTitle::default());
        }

        let text = cursor.next_text()?.map(|inner| inner.text.to_string());

        cursor.assert_element_close()?;

        Ok(MjTitle {
            children: text.unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_title::MjTitle;

    crate::should_parse!(success, MjTitle, "<mj-title>Hello World!</mj-title>");
}
