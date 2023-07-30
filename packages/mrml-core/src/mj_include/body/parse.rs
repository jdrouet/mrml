use std::convert::TryFrom;

use xmlparser::StrSpan;

use super::{MjIncludeBody, MjIncludeBodyAttributes, MjIncludeBodyChild, MjIncludeBodyKind};
use crate::comment::Comment;
use crate::mj_accordion::NAME as MJ_ACCORDION;
use crate::mj_body::MjBodyChild;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_carousel::NAME as MJ_CAROUSEL;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_group::NAME as MJ_GROUP;
use crate::mj_hero::NAME as MJ_HERO;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_navbar::NAME as MJ_NAVBAR;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_social::NAME as MJ_SOCIAL;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_table::NAME as MJ_TABLE;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::{MjWrapper, NAME as MJ_WRAPPER};
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken,
};
use crate::text::Text;

impl<'a> ElementParser<'a, MjIncludeBodyChild> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjIncludeBodyChild, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjIncludeBodyChild::MjAccordion(self.parse(tag)?)),
            MJ_BUTTON => Ok(MjIncludeBodyChild::MjButton(self.parse(tag)?)),
            MJ_CAROUSEL => Ok(MjIncludeBodyChild::MjCarousel(self.parse(tag)?)),
            MJ_COLUMN => Ok(MjIncludeBodyChild::MjColumn(self.parse(tag)?)),
            MJ_DIVIDER => Ok(MjIncludeBodyChild::MjDivider(self.parse(tag)?)),
            MJ_GROUP => Ok(MjIncludeBodyChild::MjGroup(self.parse(tag)?)),
            MJ_HERO => Ok(MjIncludeBodyChild::MjHero(self.parse(tag)?)),
            MJ_IMAGE => Ok(MjIncludeBodyChild::MjImage(self.parse(tag)?)),
            MJ_NAVBAR => Ok(MjIncludeBodyChild::MjNavbar(self.parse(tag)?)),
            MJ_RAW => Ok(MjIncludeBodyChild::MjRaw(self.parse(tag)?)),
            MJ_SECTION => Ok(MjIncludeBodyChild::MjSection(self.parse(tag)?)),
            MJ_SOCIAL => Ok(MjIncludeBodyChild::MjSocial(self.parse(tag)?)),
            MJ_SPACER => Ok(MjIncludeBodyChild::MjSpacer(self.parse(tag)?)),
            MJ_TABLE => Ok(MjIncludeBodyChild::MjTable(self.parse(tag)?)),
            MJ_TEXT => Ok(MjIncludeBodyChild::MjText(self.parse(tag)?)),
            MJ_WRAPPER => Ok(MjIncludeBodyChild::MjWrapper(self.parse(tag)?)),
            _ => Err(Error::UnexpectedElement(tag.into())),
        }
    }
}

impl<'a> TryFrom<StrSpan<'a>> for MjIncludeBodyKind {
    type Error = Error;

    fn try_from(s: StrSpan<'a>) -> Result<Self, Self::Error> {
        match s.as_str() {
            "html" => Ok(Self::Html),
            "mjml" => Ok(Self::Mjml),
            _ => Err(Error::InvalidAttribute(s.into())),
        }
    }
}

impl<'a> AttributesParser<'a, MjIncludeBodyAttributes> for MrmlParser<'a> {
    fn parse_attributes(&mut self) -> Result<MjIncludeBodyAttributes, Error> {
        let mut path = None;
        let mut kind = None;
        while let Some(attr) = self.next_attribute()? {
            match attr.local.as_str() {
                "path" => {
                    path = Some(attr.value.to_string());
                }
                "type" => {
                    kind = Some(MjIncludeBodyKind::try_from(attr.value)?);
                }
                _ => {
                    return Err(Error::UnexpectedAttribute(attr.span.into()));
                }
            }
        }
        Ok(MjIncludeBodyAttributes {
            path: path.ok_or_else(|| Error::MissingAttribute("path", Default::default()))?,
            kind: kind.unwrap_or_default(),
        })
    }
}

impl<'a> ChildrenParser<'a, Vec<MjIncludeBodyChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjIncludeBodyChild>, Error> {
        let mut result = Vec::new();

        while let Some(token) = self.next_token() {
            match token? {
                MrmlToken::Comment(inner) => {
                    result.push(MjIncludeBodyChild::Comment(Comment::from(
                        inner.text.as_str(),
                    )));
                }
                MrmlToken::ElementStart(inner) => {
                    result.push(self.parse(inner.local)?);
                }
                MrmlToken::ElementClose(inner) => {
                    self.rewind(MrmlToken::ElementClose(inner));
                    return Ok(result);
                }
                MrmlToken::Text(inner) => {
                    result.push(MjIncludeBodyChild::Text(Text::from(inner.text.as_str())));
                }
                other => return Err(Error::UnexpectedToken(other.span())),
            }
        }

        Ok(result)
    }
}

impl<'a> ElementParser<'a, MjIncludeBody> for MrmlParser<'a> {
    fn parse(&mut self, tag: StrSpan<'a>) -> Result<MjIncludeBody, Error> {
        let (attributes, children): (MjIncludeBodyAttributes, Vec<MjIncludeBodyChild>) =
            self.parse_attributes_and_children()?;

        // if a mj-include has some content, we don't load it
        let children: Vec<MjIncludeBodyChild> = if children.is_empty() {
            let child = self
                .options
                .include_loader
                .resolve(&attributes.path)
                .map_err(|source| Error::IncludeLoaderError {
                    position: tag.into(),
                    source,
                })?;
            match attributes.kind {
                MjIncludeBodyKind::Html => {
                    let children: Vec<MjBodyChild> =
                        self.new_child(child.as_str()).parse_children()?;
                    vec![MjIncludeBodyChild::MjWrapper(MjWrapper {
                        attributes: Default::default(),
                        children,
                    })]
                }
                MjIncludeBodyKind::Mjml => self.new_child(child.as_str()).parse_children()?,
            }
        } else {
            children
        };

        Ok(MjIncludeBody {
            attributes,
            children,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::convert::TryFrom;

    use xmlparser::StrSpan;

    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyKind};
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::{MrmlParser, ParserOptions};

    #[test]
    fn kind_parser() {
        assert_eq!(
            MjIncludeBodyKind::try_from(StrSpan::from("html")).unwrap(),
            MjIncludeBodyKind::Html
        );
        assert_eq!(
            MjIncludeBodyKind::try_from(StrSpan::from("mjml")).unwrap(),
            MjIncludeBodyKind::Mjml
        );
        assert!(MjIncludeBodyKind::try_from(StrSpan::from("foo")).is_err());
    }

    #[test]
    #[should_panic(expected = "InvalidAttribute(Span { start: 18, end: 21 })")]
    fn invalid_kind() {
        let raw = r#"<mj-include type="foo" path="basic.mjml" />"#;
        let _res: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(
        expected = "IncludeLoaderError { position: Span { start: 1, end: 11 }, source: IncludeLoaderError { path: \"basic.mjml\", reason: NotFound, message: None, cause: None } }"
    )]
    fn basic_in_noop_resolver() {
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let _: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn basic_with_children() {
        let raw = r#"<mj-include path="basic.mjml"><mj-text>Hello World</mj-text> <!-- Coucou --></mj-include>"#;
        let _: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    fn basic_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="basic.mjml" />"#;
        let include: MjIncludeBody = MrmlParser::new(raw, opts.into()).parse_root().unwrap();
        assert_eq!(include.attributes.kind, MjIncludeBodyKind::Mjml);
        let _content = include.children.first().unwrap();
    }

    #[test]
    fn type_html_in_memory_resolver() {
        let resolver = MemoryIncludeLoader::from(vec![("partial.html", "<h1>Hello World!</h1>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let raw = r#"<mj-include path="partial.html" type="html" />"#;
        let include: MjIncludeBody = MrmlParser::new(raw, opts.into()).parse_root().unwrap();
        assert_eq!(include.attributes.kind, MjIncludeBodyKind::Html);
        let _content = include.children.first().unwrap();
    }

    #[test]
    fn parse_all_kind_of_children() {
        let raw = r#"<mj-include path="partial.html">
    <mj-accordion />
    <mj-button />
    <mj-carousel />
    <mj-column />
    <mj-divider />
    <mj-group />
    <mj-hero />
    <mj-image path="./here.png" />
    <mj-navbar />
    <mj-raw />
    <mj-section />
    <mj-social />
    <mj-spacer />
    <mj-table />
    <mj-text />
    <mj-wrapper />
    <!-- hello -->
    World
</mj-include>"#;
        let _res: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedElement(Span { start: 38, end: 41 })")]
    fn parse_unexpected_child() {
        let raw = r#"<mj-include path="partial.html">
    <foo />
</mj-include>"#;
        let _res: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "UnexpectedAttribute(Span { start: 12, end: 31 })")]
    fn invalid_attribute() {
        let raw =
            r#"<mj-include invalid="attribute" path="partial.html"><!-- empty --></mj-include>"#;
        let _res: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "MissingAttribute(\"path\", Span { start: 0, end: 0 })")]
    fn missing_path() {
        let raw = r#"<mj-include><!-- empty --></mj-include>"#;
        let _res: MjIncludeBody = MrmlParser::new(raw, Default::default())
            .parse_root()
            .unwrap();
    }
}
