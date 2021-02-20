use super::MJBody;
use crate::elements::body::BodyElement;
use crate::elements::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default().add("width", "600px");
}

struct MJBodyParser<'a> {
    header: &'a Header,
    body: Option<MJBody>,
}

impl<'a> MJBodyParser<'a> {
    fn new(header: &'a Header) -> Self {
        Self { header, body: None }
    }

    fn default_attributes<'b>(node: &Node<'b>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'a> MJMLParser for MJBodyParser<'a> {
    type Output = MJBody;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.body.unwrap())
    }

    fn parse<'b>(mut self, node: &Node<'b>) -> Result<Self, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::parse(child, self.header, None::<&Attributes>)?);
        }
        self.body = Some(MJBody {
            attributes: Self::default_attributes(node, self.header).concat(node),
            children,
            context: None,
            exists: true,
        });
        Ok(self)
    }
}

impl MJBody {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJBody, Error> {
        MJBodyParser::new(header).parse(node)?.build()
    }
}
