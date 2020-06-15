use crate::mjml::body::prelude::*;
use crate::mjml::body::raw::RawElement;
use crate::mjml::error::Error;
use crate::mjml::prelude::*;
use crate::util::{Context, Header};
use crate::Options;
use roxmltree::Node;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct MJRaw {
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJRaw {
    pub fn parse<'a, 'b>(node: &Node<'a, 'b>, opts: &Options) -> Result<MJRaw, Error> {
        let mut children = vec![];
        for child in node.children() {
            children.push(RawElement::conditional_parse(&child, opts, true)?);
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

    fn render(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        Ok(res.join(""))
    }
}

impl BodyComponent for MJRaw {}
impl BodyContainedComponent for MJRaw {}
impl ComponentWithAttributes for MJRaw {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
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
