pub mod children;
mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;
use children::MJBodyChild;

pub const NAME: &str = "mj-body";

#[derive(Clone, Debug)]
pub struct MJBody {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJBodyChild>,
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
