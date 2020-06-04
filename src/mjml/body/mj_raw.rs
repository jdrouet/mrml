use crate::mjml::body::prelude::*;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::{Context, Header};
use crate::Options;
use crate::{close_tag, closed_tag, open_tag};
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
enum MJRawChild {
    Comment(String),
    Element(String, HashMap<String, String>, Vec<MJRawChild>),
    Text(String),
}

impl MJRawChild {
    fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJRawChild, Error> {
        if node.is_text() {
            return Ok(MJRawChild::Text(match node.text() {
                Some(value) => String::from(value),
                None => "".into(),
            }));
        }
        if node.is_comment() {
            return Ok(MJRawChild::Comment(match node.text() {
                Some(value) => String::from(value),
                None => "".into(),
            }));
        }
        if node.is_element() {
            let mut children = vec![];
            for child in node.children() {
                children.push(MJRawChild::parse(child, opts)?);
            }
            return Ok(MJRawChild::Element(
                node.tag_name().name().into(),
                get_node_attributes(&node),
                children,
            ));
        }
        Ok(MJRawChild::Text("".into()))
    }

    fn render(&self) -> String {
        match self {
            MJRawChild::Comment(content) => format!("<!-- {} -->", content),
            MJRawChild::Element(tag, attributes, children) => {
                let mut res = vec![];
                let mut attrs = vec![];
                for (key, value) in attributes.iter() {
                    attrs.push(format!("{}=\"{}\"", key, value));
                }
                if tag == "img" && children.len() == 0 {
                    res.push(closed_tag!(tag, attrs.join("")));
                } else {
                    res.push(open_tag!(tag, attrs.join("")));
                    for child in children.iter() {
                        res.push(child.render());
                    }
                    res.push(close_tag!(tag));
                }
                res.join("")
            }
            MJRawChild::Text(content) => content.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct MJRaw {
    context: Option<Context>,
    children: Vec<MJRawChild>,
}

impl MJRaw {
    pub fn parse<'a, 'b>(node: Node<'a, 'b>, opts: &Options) -> Result<MJRaw, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(MJRawChild::parse(child, opts)?);
        }
        Ok(MJRaw {
            context: None,
            children,
        })
    }
}

impl Component for MJRaw {
    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx.clone());
    }

    fn render(&self, _header: &Header) -> Result<String, Error> {
        let res: Vec<String> = self.children.iter().map(|child| child.render()).collect();
        Ok(res.join(""))
    }
}

impl BodyComponent for MJRaw {}
impl BodyContainedComponent for MJRaw {}
impl ComponentWithAttributes for MJRaw {
    fn source_attributes(&self) -> Option<&HashMap<String, String>> {
        None
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tests::compare_render;

    #[test]
    fn base() {
        compare_render(
            include_str!("../../../test/mj-raw.mjml"),
            include_str!("../../../test/mj-raw.html"),
        );
    }
}
