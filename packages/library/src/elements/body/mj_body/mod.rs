mod parser;
mod renderer;

use super::BodyElement;
use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-body";

#[derive(Clone, Debug)]
pub struct MJBody {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<BodyElement>,
    exists: bool,
}

impl MJBody {
    pub fn empty() -> MJBody {
        MJBody {
            attributes: Attributes::default(),
            children: vec![],
            context: None,
            exists: false,
        }
    }
}
