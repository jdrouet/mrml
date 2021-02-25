mod children;
mod parser;
mod renderer;

use crate::util::context::Context;
use crate::util::header::Header;
use crate::Options;
use children::MJHeadChild;
use log::debug;

pub const NAME: &str = "mj-head";

#[derive(Debug)]
pub struct MJHead {
    context: Option<Context>,
    children: Vec<MJHeadChild>,
    header: Header,
}

impl MJHead {
    pub fn empty(opts: &Options) -> MJHead {
        debug!("create empty");
        MJHead {
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
