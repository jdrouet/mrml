pub mod children;
mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;
use children::MJBodyChild;

pub const NAME: &str = "mj-body";

#[derive(Debug)]
pub struct MJBody {
    attributes: Attributes,
    context: Option<Context>,
    children: Vec<MJBodyChild>,
    exists: bool,
}

impl Default for MJBody {
    fn default() -> MJBody {
        MJBody {
            attributes: Attributes::default(),
            children: vec![],
            context: None,
            exists: false,
        }
    }
}
