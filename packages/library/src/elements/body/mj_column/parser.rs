use super::MJColumn;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("direction", "ltr")
        .add("vertical-align", "top");
}

struct MJColumnParser<'h, 'p> {
    header: &'h Header,
    extra: Option<&'p Attributes>,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h, 'p> MJColumnParser<'h, 'p> {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn new(header: &'h Header, extra: Option<&'p Attributes>) -> Self {
        Self {
            header,
            extra,
            attributes: Attributes::new(),
            children: vec![],
        }
    }
}

impl<'h, 'p> MJMLParser for MJColumnParser<'h, 'p> {
    type Output = MJColumn;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJColumn {
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        self.attributes = Self::default_attributes(node, self.header);
        if let Some(extra) = self.extra {
            self.attributes.merge(extra);
        };
        self.attributes.merge(node);
        for child in node.children.iter() {
            self.children.push(BodyElement::parse(
                &child,
                self.header,
                None::<&Attributes>,
            )?);
        }
        Ok(self)
    }
}

impl MJColumn {
    pub fn parse<'a>(
        node: &Node<'a>,
        header: &Header,
        extra: Option<&Attributes>,
    ) -> Result<MJColumn, Error> {
        MJColumnParser::new(header, extra).parse(node)?.build()
    }
}
