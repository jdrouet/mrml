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
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
};
use crate::text::Text;

impl<'a> ElementParser<'a, MjIncludeHeadChild> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjIncludeHeadChild, Error> {
        match tag.as_str() {
            MJ_ATTRIBUTES => self.parse(tag).map(MjIncludeHeadChild::MjAttributes),
            MJ_BREAKPOINT => self.parse(tag).map(MjIncludeHeadChild::MjBreakpoint),
            MJ_FONT => self.parse(tag).map(MjIncludeHeadChild::MjFont),
            MJ_PREVIEW => self.parse(tag).map(MjIncludeHeadChild::MjPreview),
            MJ_RAW => self.parse(tag).map(MjIncludeHeadChild::MjRaw),
            MJ_STYLE => self.parse(tag).map(MjIncludeHeadChild::MjStyle),
            MJ_TITLE => self.parse(tag).map(MjIncludeHeadChild::MjTitle),
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

impl<'a> AttributesParser<'a, MjIncludeHeadAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjIncludeHeadAttributes, Error> {
        let mut path = None;
        let mut kind = None;
        while let Some(attr) = self.next_attribute()? {
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

impl<'a> ChildrenParser<'a, Vec<MjIncludeHeadChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjIncludeHeadChild>, Error> {
        let mut result = Vec::new();
        while let Some(token) = self.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeHeadChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeHeadChild::Text(Text::from(inner.text.as_str())));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(inner.local)?);
                }
                MrmlToken::ElementClose(close) => {
                    self.rewind(MrmlToken::ElementClose(close));
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

impl<'a> ElementParser<'a, MjIncludeHead> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjIncludeHead, Error> {
        let attributes: MjIncludeHeadAttributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;

        let children = if ending.empty {
            Vec::new()
        } else {
            let children = self.parse_children()?;
            self.assert_element_close()?;

            children
        };

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeHeadChild> = if children.is_empty() {
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
                    vec![MjIncludeHeadChild::MjStyle(crate::mj_style::MjStyle::from(
                        child,
                    ))]
                }
                MjIncludeHeadKind::Css { inline: true } => unimplemented!(),
                MjIncludeHeadKind::Mjml => self.new_child(child.as_str()).parse_children()?,
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

    use xmlparser::StrSpan;

    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadKind};
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::{MrmlParser, ParserOptions};

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
        let _: MjIncludeHead = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 12, end: 25 })")]
    fn should_error_when_unknown_attribute() {
        let raw = r#"<mj-include unknown="yep" />"#;
        let _: MjIncludeHead = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
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
        let _: MjIncludeHead = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedElement(Span { start: 29, end: 32 })")]
    fn should_error_unknown_children() {
        let raw = r#"<mj-include path="inmemory"><div /></mj-include>"#;
        let _: MjIncludeHead = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "IncludeLoaderError { position: Span { start: 1, end: 11 }, source: IncludeLoaderError { path: \"basic.mjml\", reason: NotFound, message: None, cause: None } }"
    )]
    fn basic_in_noop_resolver() {
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let _: MjIncludeHead = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn basic_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-title>Hello</mj-title>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let include: MjIncludeHead = MrmlParser::new(raw, opts.into()).parse_root().unwrap();
        assert_eq!(include.attributes.kind, MjIncludeHeadKind::Mjml);
        let _content = include.children.first().unwrap();
    }

    #[test]
    fn type_css_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("partial.css", "* { background-color: red; }")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="partial.css" type="css" />"#;
        let include: MjIncludeHead = MrmlParser::new(raw, opts.into()).parse_root().unwrap();
        assert_eq!(
            include.attributes.kind,
            MjIncludeHeadKind::Css { inline: false }
        );
        let _content = include.children.first().unwrap();
    }
}
