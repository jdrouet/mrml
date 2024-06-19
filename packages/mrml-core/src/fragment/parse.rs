use xmlparser::StrSpan;

use super::Fragment;
#[cfg(feature = "async")]
use crate::prelude::parser::{AsyncMrmlParser, AsyncParseChildren, AsyncParseElement};
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseChildren, ParseElement};

impl<'opts, T> ParseElement<Fragment<T>> for MrmlParser<'opts>
where
    MrmlParser<'opts>: ParseChildren<Vec<T>>,
{
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<Fragment<T>, Error> {
        let children = self.parse_children(cursor)?;

        cursor.assert_element_close()?;

        Ok(Fragment { children })
    }
}

#[cfg(feature = "async")]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl<T> AsyncParseElement<Fragment<T>> for AsyncMrmlParser
where
    AsyncMrmlParser: AsyncParseChildren<Vec<T>>,
{
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        _tag: StrSpan<'a>,
    ) -> Result<Fragment<T>, Error> {
        let children = self.async_parse_children(cursor).await?;

        cursor.assert_element_close()?;

        Ok(Fragment { children })
    }
}
