use std::convert::TryFrom;

use xmlparser::StrSpan;

use super::{MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild, MjIncludeHeadKind};
use crate::comment::Comment;
use crate::mj_attributes::NAME as MJ_ATTRIBUTES;
use crate::mj_breakpoint::NAME as MJ_BREAKPOINT;
use crate::mj_font::NAME as MJ_FONT;
use crate::mj_preview::NAME as MJ_PREVIEW;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_style::NAME as MJ_STYLE;
use crate::mj_title::NAME as MJ_TITLE;
use crate::prelude::parser::{
    AsyncParseElement, Error, MrmlCursor, MrmlParser, MrmlToken, ParseAttributes, ParseChildren,
    ParseElement,
};
use crate::text::Text;

impl ParseElement<MjIncludeHeadChild> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self
                .parse(cursor, tag)
                .map(MjIncludeHeadChild::MjAttributes),
            MJ_BREAKPOINT => self
                .parse(cursor, tag)
                .map(MjIncludeHeadChild::MjBreakpoint),
            MJ_FONT => self.parse(cursor, tag).map(MjIncludeHeadChild::MjFont),
            MJ_PREVIEW => self.parse(cursor, tag).map(MjIncludeHeadChild::MjPreview),
            MJ_RAW => self.parse(cursor, tag).map(MjIncludeHeadChild::MjRaw),
            MJ_STYLE => self.parse(cursor, tag).map(MjIncludeHeadChild::MjStyle),
            MJ_TITLE => self.parse(cursor, tag).map(MjIncludeHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

impl ParseAttributes<MjIncludeHeadAttributes> for MrmlParser {
    fn parse_attributes<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<MjIncludeHeadAttributes, Error> {
        let mut path = None;
        let mut kind = None;
        while let Some(attr) = cursor.next_attribute()? {
            match attr.local.as_str() {
                "path" => {
                    path = Some(attr.value.to_string());
                }
                "type" => {
                    kind = Some(MjIncludeHeadKind::try_from(attr.value)?);
                }
                _ => {
                    return Err(Error::UnexpectedAttribute(attr.span.into()));
                }
            }
        }
        Ok(MjIncludeHeadAttributes {
            path: path.ok_or_else(|| Error::MissingAttribute("path", Default::default()))?,
            kind: kind.unwrap_or_default(),
        })
    }
}

impl ParseChildren<Vec<MjIncludeHeadChild>> for MrmlParser {
    fn parse_children<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
    ) -> Result<Vec<MjIncludeHeadChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = cursor.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeHeadChild::Comment(Comment::from(inner.text.as_str())));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeHeadChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(cursor, inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    cursor.rewind(MrmlToken::ElementClose(close));
                    return Ok(result);
                }
                other => {
                    return Err(Error::UnexpectedToken(other.span()));
                }
            }
        }
        Ok(result)
    }
}

impl ParseElement<MjIncludeHead> for MrmlParser {
    fn parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHead, Error> {
        let (attributes, children): (MjIncludeHeadAttributes, Vec<MjIncludeHeadChild>) =
            self.parse_attributes_and_children(cursor)?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeHeadChild> =
            if children.is_empty() {
                let child = self
                    .options
                    .include_loader
                    .resolve(&attributes.path)
                    .map_err(|source| Error::IncludeLoaderError {
                        position: tag.into(),
                        source,
                    })?;

                match attributes.kind {
                    MjIncludeHeadKind::Css { inline: false } => {
                        vec![MjIncludeHeadChild::MjStyle(crate::mj_style::MjStyle::from(child))]
                    }
                    MjIncludeHeadKind::Css { inline: true } => unimplemented!(),
                    MjIncludeHeadKind::Mjml => {
                        let mut sub = cursor.new_child(child.as_str());
                        self.parse_children(&mut sub)?
                    }
                    MjIncludeHeadKind::Html => todo!(),
                }
            } else {
                children
            };

        Ok(MjIncludeHead {
            attributes,
            children,
        })
    }
}

#[async_trait::async_trait]
impl AsyncParseElement<MjIncludeHead> for MrmlParser {
    async fn async_parse<'a>(
        &self,
        cursor: &mut MrmlCursor<'a>,
        tag: StrSpan<'a>,
    ) -> Result<MjIncludeHead, Error> {
        let (attributes, children): (MjIncludeHeadAttributes, Vec<MjIncludeHeadChild>) =
            self.parse_attributes_and_children(cursor)?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeHeadChild> =
            if children.is_empty() {
                let child = self
                    .options
                    .include_loader
                    .resolve(&attributes.path)
                    .map_err(|source| Error::IncludeLoaderError {
                        position: tag.into(),
                        source,
                    })?;

                match attributes.kind {
                    MjIncludeHeadKind::Css { inline: false } => {
                        vec![MjIncludeHeadChild::MjStyle(crate::mj_style::MjStyle::from(child))]
                    }
                    MjIncludeHeadKind::Css { inline: true } => unimplemented!(),
                    MjIncludeHeadKind::Mjml => {
                        let mut sub = cursor.new_child(child.as_str());
                        self.parse_children(&mut sub)?
                    }
                    MjIncludeHeadKind::Html => todo!(),
                }
            } else {
                children
            };

        Ok(MjIncludeHead {
            attributes,
            children,
        })
    }
}

impl<'a> TryFrom<StrSpan<'a>> for MjIncludeHeadKind {
    type Error = Error;

    fn try_from(s: StrSpan<'a>) -> Result<Self, Self::Error> {
        match s.as_str() {
            "html" => Ok(Self::Html),
            "mjml" => Ok(Self::Mjml),
            "css" => Ok(Self::Css { inline: false }),
            _ => Err(Error::InvalidAttribute(s.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;
    use std::sync::Arc;

    use xmlparser::StrSpan;

    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadKind};
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::{MrmlCursor, MrmlParser, ParserOptions};

    #[test]
    fn should_parse_every_kind() {
        assert_eq!(
            MjIncludeHeadKind::try_from(StrSpan::from("html")).unwrap(),
            MjIncludeHeadKind::Html
        );
        assert_eq!(
            MjIncludeHeadKind::try_from(StrSpan::from("mjml")).unwrap(),
            MjIncludeHeadKind::Mjml
        );
        assert_eq!(
            MjIncludeHeadKind::try_from(StrSpan::from("css")).unwrap(),
            MjIncludeHeadKind::Css { inline: false }
        );
        assert!(MjIncludeHeadKind::try_from(StrSpan::from("other")).is_err());
    }

    #[test]
    #[should_panic(expected = "MissingAttribute(\"path\", Span { start: 0, end: 0 })")]
    fn should_error_when_no_path() {
        let raw = r#"<mj-include />"#;
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(raw);
        let _: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 12, end: 25 })")]
    fn should_error_when_unknown_attribute() {
        let raw = r#"<mj-include unknown="yep" />"#;
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(raw);
        let _: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
    }

    #[test]
    fn should_parse_all_children() {
        let raw = r#"<mj-include path="inmemory">
    <mj-attributes />
    <mj-breakpoint width="400px" />
    <mj-font name="Comic" href="..." />
    <mj-preview>Preview</mj-preview>
    <mj-raw />
    <mj-style />
    <mj-title>Title</mj-title>
    <!-- Comment -->
</mj-include>"#;
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(raw);
        let _: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedElement(Span { start: 29, end: 32 })")]
    fn should_error_unknown_children() {
        let raw = r#"<mj-include path="inmemory"><div /></mj-include>"#;
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(raw);
        let _: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
    }

    #[test]
    #[should_panic(
        expected = "IncludeLoaderError { position: Span { start: 1, end: 11 }, source: IncludeLoaderError { path: \"basic.mjml\", reason: NotFound, message: None, cause: None } }"
    )]
    fn basic_in_noop_resolver() {
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let parser = MrmlParser::default();
        let mut cursor = MrmlCursor::new(raw);
        let _: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
    }

    #[test]
    fn basic_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-title>Hello</mj-title>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let parser = MrmlParser::new(Arc::new(opts));
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(include.attributes.kind, MjIncludeHeadKind::Mjml);
        let _content = include.children.first().unwrap();
    }

    #[test]
    fn type_css_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("partial.css", "* { background-color: red; }")]);
        let raw = r#"<mj-include path="partial.css" type="css" />"#;
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let parser = MrmlParser::new(Arc::new(opts));
        let mut cursor = MrmlCursor::new(raw);
        let include: MjIncludeHead = parser.parse_root(&mut cursor).unwrap();
        assert_eq!(
            include.attributes.kind,
            MjIncludeHeadKind::Css { inline: false }
        );
        let _content = include.children.first().unwrap();
    }
}
