use crate::elements::body::prelude::*;
use crate::elements::body::raw::RawElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::Attributes;
use crate::util::{Context, Header};

#[derive(Clone, Debug)]
pub struct MJRaw {
    context: Option<Context>,
    children: Vec<RawElement>,
}

impl MJRaw {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJRaw, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(RawElement::conditional_parse(&child, header, true)?);
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
    fn attributes(&self) -> Option<&Attributes> {
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
