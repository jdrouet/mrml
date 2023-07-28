use std::convert::TryFrom;
use std::rc::Rc;
use std::str::FromStr;

use xmlparser::{StrSpan, Tokenizer};

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
use crate::mj_wrapper::MjWrapper;
use crate::mj_wrapper::NAME as MJ_WRAPPER;
use crate::prelude::parser::{
    AttributesParser, ChildrenParser, ElementParser, Error, MrmlParser, MrmlToken, Parsable,
    Parser, ParserOptions,
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
            _ => Err(Error::UnexpectedElement(tag.start())),
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
                "kind" => {
                    kind = Some(MjIncludeBodyKind::from_str(attr.value.as_str())?);
                }
                _ => {
                    return Err(Error::UnexpectedAttribute(attr.span.start()));
                }
            }
        }
        Ok(MjIncludeBodyAttributes {
            path: path.ok_or_else(|| Error::MissingAttribute("path"))?,
            kind: kind.ok_or_else(|| Error::MissingAttribute("kind"))?,
        })
    }
}

impl<'a> ChildrenParser<'a, Vec<MjIncludeBodyChild>> for MrmlParser<'a> {
    fn parse_children(&mut self) -> Result<Vec<MjIncludeBodyChild>, Error> {
        let mut result = Vec::new();

        loop {
            match self.assert_next()? {
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
                other => return Err(Error::unexpected_token(other.range())),
            }
        }
    }
}

impl<'a> ElementParser<'a, MjIncludeBody> for MrmlParser<'a> {
    fn parse(&mut self, _tag: StrSpan<'a>) -> Result<MjIncludeBody, Error> {
        let attributes: MjIncludeBodyAttributes = self.parse_attributes()?;
        let ending = self.assert_element_end()?;

        let children = if ending.empty {
            let children = self.parse_children()?;
            self.assert_element_close()?;

            children
        } else {
            Vec::new()
        };

        // if a mj-include has some content, we don't load it
        let children = if children.is_empty() {
            let child = self.options.include_loader.resolve(&attributes.path)?;
            let children: Vec<MjIncludeBodyChild> =
                self.new_child(child.as_str()).parse_children()?;
            attributes.kind.wrap(children)?
        } else {
            children
        };

        Ok(MjIncludeBody {
            attributes,
            children,
        })
    }
}

impl From<Comment> for MjIncludeBodyChild {
    fn from(value: Comment) -> Self {
        Self::Comment(value)
    }
}

impl From<Text> for MjIncludeBodyChild {
    fn from(value: Text) -> Self {
        Self::Text(value)
    }
}

impl std::convert::TryFrom<MjIncludeBodyChild> for MjBodyChild {
    type Error = Error;

    fn try_from(value: MjIncludeBodyChild) -> Result<Self, Error> {
        Ok(match value {
            MjIncludeBodyChild::Comment(inner) => MjBodyChild::Comment(inner),
            MjIncludeBodyChild::MjAccordion(inner) => MjBodyChild::MjAccordion(inner),
            MjIncludeBodyChild::MjButton(inner) => MjBodyChild::MjButton(inner),
            MjIncludeBodyChild::MjCarousel(inner) => MjBodyChild::MjCarousel(inner),
            MjIncludeBodyChild::MjColumn(inner) => MjBodyChild::MjColumn(inner),
            MjIncludeBodyChild::MjDivider(inner) => MjBodyChild::MjDivider(inner),
            MjIncludeBodyChild::MjGroup(inner) => MjBodyChild::MjGroup(inner),
            MjIncludeBodyChild::MjHero(inner) => MjBodyChild::MjHero(inner),
            MjIncludeBodyChild::MjImage(inner) => MjBodyChild::MjImage(inner),
            MjIncludeBodyChild::MjNavbar(inner) => MjBodyChild::MjNavbar(inner),
            MjIncludeBodyChild::MjRaw(inner) => MjBodyChild::MjRaw(inner),
            MjIncludeBodyChild::MjSection(inner) => MjBodyChild::MjSection(inner),
            MjIncludeBodyChild::MjSocial(inner) => MjBodyChild::MjSocial(inner),
            MjIncludeBodyChild::MjSpacer(inner) => MjBodyChild::MjSpacer(inner),
            MjIncludeBodyChild::MjTable(inner) => MjBodyChild::MjTable(inner),
            MjIncludeBodyChild::MjText(inner) => MjBodyChild::MjText(inner),
            MjIncludeBodyChild::MjWrapper(inner) => MjBodyChild::MjWrapper(inner),
            MjIncludeBodyChild::Node(inner) => MjBodyChild::Node(inner),
            MjIncludeBodyChild::Text(inner) => MjBodyChild::Text(inner),
        })
    }
}

impl Parsable for MjIncludeBodyChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            crate::mj_accordion::NAME => Ok(Self::MjAccordion(
                crate::mj_accordion::MjAccordion::parse(tag, tokenizer, opts)?,
            )),
            crate::mj_button::NAME => Ok(Self::MjButton(crate::mj_button::MjButton::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_carousel::NAME => Ok(Self::MjCarousel(
                crate::mj_carousel::MjCarousel::parse(tag, tokenizer, opts)?,
            )),
            crate::mj_column::NAME => Ok(Self::MjColumn(crate::mj_column::MjColumn::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_divider::NAME => Ok(Self::MjDivider(crate::mj_divider::MjDivider::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_group::NAME => Ok(Self::MjGroup(crate::mj_group::MjGroup::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_hero::NAME => Ok(Self::MjHero(crate::mj_hero::MjHero::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_image::NAME => Ok(Self::MjImage(crate::mj_image::MjImage::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_navbar::NAME => Ok(Self::MjNavbar(crate::mj_navbar::MjNavbar::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_raw::NAME => Ok(Self::MjRaw(crate::mj_raw::MjRaw::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_section::NAME => Ok(Self::MjSection(crate::mj_section::MjSection::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_social::NAME => Ok(Self::MjSocial(crate::mj_social::MjSocial::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_spacer::NAME => Ok(Self::MjSpacer(crate::mj_spacer::MjSpacer::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_table::NAME => Ok(Self::MjTable(crate::mj_table::MjTable::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_text::NAME => Ok(Self::MjText(crate::mj_text::MjText::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_wrapper::NAME => Ok(Self::MjWrapper(crate::mj_wrapper::MjWrapper::parse(
                tag, tokenizer, opts,
            )?)),
            _ => Ok(Self::Node(crate::node::Node::parse(tag, tokenizer, opts)?)),
        }
    }
}

impl FromStr for MjIncludeBodyKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(Self::Html),
            "mjml" => Ok(Self::Mjml),
            other => Err(Error::InvalidElement(format!(
                "invalid mj-include attribute kind {other:?}"
            ))),
        }
    }
}

impl MjIncludeBodyKind {
    fn wrap(&self, children: Vec<MjIncludeBodyChild>) -> Result<Vec<MjIncludeBodyChild>, Error> {
        match self {
            Self::Mjml => Ok(children),
            Self::Html => {
                let mut wrapper = MjWrapper::default();
                for child in children {
                    wrapper.children.push(MjBodyChild::try_from(child)?);
                }
                Ok(vec![MjIncludeBodyChild::MjWrapper(wrapper)])
            }
        }
    }
}

#[derive(Debug)]
struct MjIncludeBodyParser {
    opts: Rc<ParserOptions>,
    attributes: MjIncludeBodyAttributes,
}

impl MjIncludeBodyParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            attributes: MjIncludeBodyAttributes::default(),
        }
    }
}

impl Parser for MjIncludeBodyParser {
    type Output = MjIncludeBody;

    fn build(self) -> Result<Self::Output, Error> {
        let child = self
            .opts
            .include_loader
            .resolve(&self.attributes.path)
            .map_err(Error::IncludeLoaderError)?;

        let child = crate::prelude::parser::loader::parse(&child, self.opts.clone())?;

        let children = self.attributes.kind.wrap(vec![child])?;

        Ok(MjIncludeBody {
            attributes: self.attributes,
            children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        match name.as_str() {
            "path" => {
                self.attributes.path = value.to_string();
            }
            "type" => {
                self.attributes.kind = MjIncludeBodyKind::from_str(value.as_str())?;
            }
            _ => return Err(Error::UnexpectedAttribute(name.start())),
        }
        Ok(())
    }
}

impl Parsable for MjIncludeBody {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjIncludeBodyParser::new(opts).parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::mj_include::body::MjIncludeBodyKind;
    use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parser::{Error, ParserOptions};

    #[test]
    fn basic_in_noop_resolver() {
        let json = r#"<mjml>
  <mj-body>
    <mj-include path="basic.mjml" />
  </mj-body>
</mjml>
"#;
        let err = crate::mjml::Mjml::parse(json).unwrap_err();
        match err {
            Error::IncludeLoaderError(origin) => {
                assert_eq!(origin.reason, std::io::ErrorKind::NotFound);
            }
            _ => panic!("expected a IncludeLoaderError"),
        }
    }

    #[test]
    fn basic_in_memory_resolver() {
        let resolver =
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let json = r#"<mjml>
  <mj-body>
    <mj-include path="basic.mjml" />
  </mj-body>
</mjml>
"#;
        let root = crate::mjml::Mjml::parse_with_options(json, Rc::new(opts)).unwrap();
        let body = root.children.body.unwrap();
        let include = body.children.first().unwrap().as_mj_include().unwrap();
        assert_eq!(include.attributes.kind, MjIncludeBodyKind::Mjml);
        let _content = include.children.first().unwrap();
    }

    #[test]
    fn type_html_in_memory_resolver() {
        let resolver = MemoryIncludeLoader::from(vec![("partial.html", "<h1>Hello World!</h1>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let json = r#"<mjml>
  <mj-body>
    <mj-include path="partial.html" type="html" />
  </mj-body>
</mjml>
"#;
        let root = crate::mjml::Mjml::parse_with_options(json, Rc::new(opts)).unwrap();
        let body = root.children.body.unwrap();
        let include = body.children.first().unwrap().as_mj_include().unwrap();
        assert_eq!(include.attributes.kind, MjIncludeBodyKind::Html);
        let _content = include.children.first().unwrap();
    }
}
