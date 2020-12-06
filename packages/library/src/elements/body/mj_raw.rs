use crate::elements::body::prelude::*;
use crate::elements::body::raw::RawElement;
use crate::elements::body::BodyElement;
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::parser::Node;
use crate::util::attributes::Attributes;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;

#[derive(Clone, Debug)]
pub struct MJRaw {
    context: Option<Context>,
    children: Vec<BodyElement>,
}

impl MJRaw {
    pub fn parse<'a>(node: &Node<'a>, header: &Header) -> Result<MJRaw, Error> {
        let mut children = vec![];
        for child in node.children.iter() {
            children.push(BodyElement::Raw(RawElement::conditional_parse(
                &child, header, true,
            )?));
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
        self.context = Some(ctx);
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let mut res = vec![];
        for child in self.children.iter() {
            res.push(child.render(header)?);
        }
        Ok(res.join(""))
    }
}

impl BodyComponent for MJRaw {
    fn attributes(&self) -> Option<&Attributes> {
        None
    }

    fn get_children(&self) -> &Vec<BodyElement> {
        &self.children
    }

    fn get_current_width(&self) -> Option<Size> {
        None
    }
}
