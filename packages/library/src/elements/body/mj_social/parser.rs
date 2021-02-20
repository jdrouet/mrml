use super::MJSocial;
use crate::elements::body::mj_social_element::{MJSocialElement, NAME as MJ_SOCIAL_ELEMENT};
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};
use crate::util::attributes::*;
use crate::util::header::Header;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("border-radius", "3px")
        .add("color", "#333333")
        .add("font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("font-size", "13px")
        .add("icon-size", "20px")
        .add("line-height", "22px")
        .add("mode", "horizontal")
        .add("padding", "10px 25px")
        .add("text-decoration", "none");
}

struct MJSocialParser<'h> {
    header: &'h Header,
    content: Option<MJSocial>,
}

impl<'h> MJSocialParser<'h> {
    pub fn new(header: &'h Header) -> Self {
        Self {
            header,
            content: None,
        }
    }

    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }
}

impl<'h> MJMLParser for MJSocialParser<'h> {
    type Output = MJSocial;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.content.unwrap())
    }

    fn parse<'a>(mut self, node: &Node<'a>) -> Result<Self, Error> {
        let mut result = MJSocial {
            attributes: Self::default_attributes(node, self.header).concat(node),
            context: None,
            children: vec![],
        };
        let attrs = result.get_children_attributes();
        for child in node.children.iter() {
            if let Some(child_node) = child.as_node() {
                let tag_name = child_node.name.as_str();
                if tag_name != MJ_SOCIAL_ELEMENT {
                    return Err(Error::UnexpectedElement(tag_name.into()));
                } else {
                    // TODO remove Option on extra attribute
                    let element = MJSocialElement::parse(&child_node, self.header, Some(&attrs))?;
                    result.children.push(BodyElement::MJSocialElement(element));
                }
            }
        }
        self.content = Some(result);
        Ok(self)
    }
}

impl MJSocial {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSocial, Error> {
        MJSocialParser::new(header).parse(node)?.build()
    }
}
