use super::error::Error;
use crate::parser::Node;
use crate::util::context::Context;
use crate::util::header::Header;
use std::collections::HashMap;

pub fn get_node_attributes(node: &Node) -> HashMap<String, String> {
    let mut res = HashMap::<String, String>::new();
    add_node_attributes(&mut res, node);
    res
}

pub fn add_node_attributes<'a>(res: &mut HashMap<String, String>, node: &Node<'a>) {
    for (key, value) in node.attributes.iter() {
        res.insert(key.as_str().into(), value.as_str().into());
    }
}

pub trait Component {
    fn context(&self) -> Option<&Context>;
    fn set_context(&mut self, ctx: Context);

    fn update_header(&self, _header: &mut Header) {}
    fn render(&self, header: &Header) -> Result<String, Error>;
}
