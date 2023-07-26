use std::convert::TryFrom;
use std::rc::Rc;
use std::str::FromStr;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjIncludeBody, MjIncludeBodyAttributes, MjIncludeBodyChild, MjIncludeBodyKind};
use crate::comment::Comment;
use crate::mj_body::MjBodyChild;
use crate::mj_wrapper::MjWrapper;
use crate::prelude::parse::{Error, Parsable, Parser, ParserOptions};
use crate::text::Text;

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

        let child = crate::prelude::parse::loader::parse(&child, self.opts.clone())?;

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
    use crate::prelude::parse::memory_loader::MemoryIncludeLoader;
    use crate::prelude::parse::{Error, ParserOptions};

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
