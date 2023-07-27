use std::rc::Rc;
use std::str::FromStr;

use xmlparser::{StrSpan, Tokenizer};

use super::{MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild, MjIncludeHeadKind};
use crate::mj_head::MjHeadChild;
use crate::prelude::parser::{Error, Parsable, Parser, ParserOptions};

impl std::convert::TryFrom<MjIncludeHeadChild> for MjHeadChild {
    type Error = Error;

    fn try_from(value: MjIncludeHeadChild) -> Result<Self, Self::Error> {
        match value {
            MjIncludeHeadChild::Comment(inner) => Ok(Self::Comment(inner)),
            MjIncludeHeadChild::MjAttributes(inner) => Ok(Self::MjAttributes(inner)),
            MjIncludeHeadChild::MjBreakpoint(inner) => Ok(Self::MjBreakpoint(inner)),
            MjIncludeHeadChild::MjFont(inner) => Ok(Self::MjFont(inner)),
            MjIncludeHeadChild::MjPreview(inner) => Ok(Self::MjPreview(inner)),
            MjIncludeHeadChild::MjRaw(inner) => Ok(Self::MjRaw(inner)),
            MjIncludeHeadChild::MjStyle(inner) => Ok(Self::MjStyle(inner)),
            MjIncludeHeadChild::MjTitle(inner) => Ok(Self::MjTitle(inner)),
            MjIncludeHeadChild::Text(_inner) => Err(Error::InvalidElement(
                "Text element should be wrapped in another element".into(),
            )),
        }
    }
}

impl Parsable for MjIncludeHeadChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            crate::mj_attributes::NAME => Ok(Self::MjAttributes(
                crate::mj_attributes::MjAttributes::parse(tag, tokenizer, opts)?,
            )),
            crate::mj_breakpoint::NAME => Ok(Self::MjBreakpoint(
                crate::mj_breakpoint::MjBreakpoint::parse(tag, tokenizer, opts)?,
            )),
            crate::mj_font::NAME => Ok(Self::MjFont(crate::mj_font::MjFont::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_preview::NAME => Ok(Self::MjPreview(crate::mj_preview::MjPreview::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_raw::NAME => Ok(Self::MjRaw(crate::mj_raw::MjRaw::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_style::NAME => Ok(Self::MjStyle(crate::mj_style::MjStyle::parse(
                tag, tokenizer, opts,
            )?)),
            crate::mj_title::NAME => Ok(Self::MjTitle(crate::mj_title::MjTitle::parse(
                tag, tokenizer, opts,
            )?)),
            _ => Err(Error::UnexpectedElement(tag.start())),
        }
    }
}

impl FromStr for MjIncludeHeadKind {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "html" => Ok(Self::Html),
            "mjml" => Ok(Self::Mjml),
            "css" => Ok(Self::Css { inline: false }),
            other => Err(Error::InvalidElement(format!(
                "invalid mj-include attribute kind {other:?}"
            ))),
        }
    }
}

#[derive(Debug)]
struct MjIncludeHeadParser {
    opts: Rc<ParserOptions>,
    attributes: MjIncludeHeadAttributes,
}

impl MjIncludeHeadParser {
    fn new(opts: Rc<ParserOptions>) -> Self {
        Self {
            opts,
            attributes: MjIncludeHeadAttributes::default(),
        }
    }
}

impl Parser for MjIncludeHeadParser {
    type Output = MjIncludeHead;

    fn build(self) -> Result<Self::Output, Error> {
        let child = self
            .opts
            .include_loader
            .resolve(&self.attributes.path)
            .map_err(Error::IncludeLoaderError)?;

        let children = match self.attributes.kind {
            MjIncludeHeadKind::Css { inline: false } => {
                vec![MjIncludeHeadChild::MjStyle(crate::mj_style::MjStyle::from(
                    child,
                ))]
            }
            MjIncludeHeadKind::Css { inline: true } => todo!(),
            MjIncludeHeadKind::Mjml => {
                let child = crate::prelude::parser::loader::parse::<MjIncludeHeadChild>(
                    &child,
                    self.opts.clone(),
                )?;
                vec![child]
            }
            MjIncludeHeadKind::Html => todo!(),
        };

        Ok(MjIncludeHead {
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
                self.attributes.kind = MjIncludeHeadKind::from_str(value.as_str())?;
            }
            "css-inline" => {
                self.attributes.kind = MjIncludeHeadKind::Css { inline: true };
            }
            _ => return Err(Error::UnexpectedAttribute(name.start())),
        }
        Ok(())
    }
}

impl Parsable for MjIncludeHead {
    fn parse(
        _tag: StrSpan,
        tokenizer: &mut Tokenizer,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        MjIncludeHeadParser::new(opts).parse(tokenizer)?.build()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::mj_include::head::MjIncludeHeadKind;
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
            MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-title>Hello</mj-title>")]);
        let opts = ParserOptions {
            include_loader: Box::new(resolver),
        };
        let json = r#"<mjml>
  <mj-head>
    <mj-include path="basic.mjml" />
  </mj-head>
  <mj-body></mj-body>
</mjml>
"#;
        let root = crate::mjml::Mjml::parse_with_options(json, Rc::new(opts)).unwrap();
        let head = root.children.head.unwrap();
        let include = head.children.first().unwrap().as_mj_include().unwrap();
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
        let json = r#"<mjml>
  <mj-head>
    <mj-include path="partial.css" type="css" />
  </mj-head>
  <mj-body></mj-body>
</mjml>
"#;
        let root = crate::mjml::Mjml::parse_with_options(json, Rc::new(opts)).unwrap();
        let head = root.children.head.unwrap();
        let include = head.children.first().unwrap().as_mj_include().unwrap();
        assert_eq!(
            include.attributes.kind,
            MjIncludeHeadKind::Css { inline: false }
        );
        let _content = include.children.first().unwrap();
    }
}
