use super::MJSocial;
use crate::elements::body::mj_social_element::MJSocialElement;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
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

impl MJSocial {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJSocial, Error> {
        let mut result = MJSocial {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children: vec![],
        };
        let attrs = result.get_children_attributes();
        for child in node.children.iter() {
            if let Some(child_node) = child.as_node() {
                let tag_name = child_node.name.as_str();
                if tag_name != "mj-social-element" {
                    return Err(Error::UnexpectedElement(tag_name.into()));
                } else {
                    let element =
                        MJSocialElement::parse_social_child(&child_node, header, Some(&attrs))?;
                    result.children.push(BodyElement::MJSocialElement(element));
                }
            }
        }
        Ok(result)
    }
}
