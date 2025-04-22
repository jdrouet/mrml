#[cfg(feature = "async")]
use crate::prelude::parser::AsyncMrmlParser;
use crate::prelude::parser::{Error, MrmlCursor, MrmlParser, ParseAttributes, WarningKind};
use htmlparser::StrSpan;

use super::MjHtmlAttributeAttributes;

#[inline(always)]
fn parse_attributes(
    cursor: &mut MrmlCursor<'_>,
    tag: &StrSpan<'_>,
) -> Result<MjHtmlAttributeAttributes, Error> {
    let mut name = None;

    while let Some(attr) = cursor.next_attribute()? {
        match (attr.local.as_str(), attr.value) {
            ("name", Some(value)) => {
                name = Some(value.to_string());
            }
            _ => {
                cursor.add_warning(WarningKind::UnexpectedAttribute, attr.span);
            }
        }
    }

    Ok(MjHtmlAttributeAttributes {
        name: name.ok_or_else(|| Error::MissingAttribute {
            name: "name",
            origin: cursor.origin(),
            position: tag.into(),
        })?,
    })
}

impl ParseAttributes<MjHtmlAttributeAttributes> for MrmlParser<'_> {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        tag: &StrSpan<'_>,
    ) -> Result<MjHtmlAttributeAttributes, Error> {
        parse_attributes(cursor, tag)
    }
}

#[cfg(feature = "async")]
impl ParseAttributes<MjHtmlAttributeAttributes> for AsyncMrmlParser {
    fn parse_attributes(
        &self,
        cursor: &mut MrmlCursor<'_>,
        tag: &StrSpan<'_>,
    ) -> Result<MjHtmlAttributeAttributes, Error> {
        parse_attributes(cursor, tag)
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_html_attribute::MjHtmlAttribute;

    crate::should_parse!(
        basic_with_children,
        MjHtmlAttribute,
        r#"<mj-html-attribute name="data-id">42</mj-html-attribute>"#
    );

    crate::should_not_parse!(
        missing_attribute,
        MjHtmlAttribute,
        r#"<mj-html-attribute>42</mj-html-attribute>"#,
        r#"MissingAttribute { name: "name", origin: Root, position: Span { start: 1, end: 18 } }"#
    );
}
