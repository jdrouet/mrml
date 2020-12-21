use super::MJNavbar;
use crate::elements::body::mj_navbar_link::MJNavbarLink;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::parser::Node;
use crate::util::attributes::*;
use crate::util::header::Header;
use crate::util::id::Generator as IdGenerator;

lazy_static! {
    static ref DEFAULT_ATTRIBUTES: Attributes = Attributes::default()
        .add("align", "center")
        .add("ico-align", "center")
        .add("ico-open", "&#9776;")
        .add("ico-close", "&#8855;")
        .add("ico-color", "#000000")
        .add("ico-font-family", "Ubuntu, Helvetica, Arial, sans-serif")
        .add("ico-font-size", "30px")
        .add("ico-text-transform", "uppercase")
        .add("ico-padding", "10px")
        .add("ico-text-decoration", "none")
        .add("ico-line-height", "30px");
}

impl MJNavbar {
    fn default_attributes<'a>(node: &Node<'a>, header: &Header) -> Attributes {
        header
            .default_attributes
            .get_attributes(node, DEFAULT_ATTRIBUTES.clone())
    }

    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJNavbar, Error> {
        let generate: IdGenerator = header.id_generator;
        let mut result = MJNavbar {
            attributes: Self::default_attributes(node, header).concat(node),
            context: None,
            children: vec![],
            id: generate(8),
        };
        let attrs = result.get_children_attributes();
        for child in node.children.iter() {
            if let Some(child_node) = child.as_node() {
                let tag_name = child_node.name.as_str();
                if tag_name != "mj-navbar-link" {
                    return Err(Error::UnexpectedElement(tag_name.into()));
                } else {
                    let element = MJNavbarLink::parse_link(&child_node, header, Some(&attrs))?;
                    result.children.push(BodyElement::MJNavbarLink(element));
                }
            }
        }
        Ok(result)
    }
}
