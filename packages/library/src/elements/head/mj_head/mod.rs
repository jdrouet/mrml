mod parser;
mod renderer;

use super::HeadElement;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::Options;
use log::debug;
use std::collections::HashMap;

pub const NAME: &str = "mj-head";

#[derive(Debug, Clone)]
pub struct MJHead {
    attributes: HashMap<String, String>,
    context: Option<Context>,
    children: Vec<HeadElement>,
    header: Header,
}

impl MJHead {
    pub fn empty(opts: Options) -> MJHead {
        debug!("create empty");
        MJHead {
            attributes: HashMap::new(),
            context: None,
            children: vec![],
            header: Header::from(opts),
        }
    }

    pub fn get_header(&self) -> &Header {
        &self.header
    }

    pub fn get_mut_header(&mut self) -> &mut Header {
        &mut self.header
    }
}
