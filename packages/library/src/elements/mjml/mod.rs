use crate::elements::body::mj_body::MJBody;
use crate::elements::head::mj_head::MJHead;
use crate::util::context::Context;

mod parser;
mod renderer;

#[derive(Debug)]
pub struct MJMLElement {
    context: Option<Context>,
    head: MJHead,
    body: MJBody,
}
