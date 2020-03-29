mod parser;
mod renderer;

use crate::util::header::DefaultAttributes;

pub const NAME: &str = "mj-attributes";

#[derive(Clone, Debug)]
pub struct MJAttributes(DefaultAttributes);

impl MJAttributes {
    fn new() -> Self {
        Self(DefaultAttributes::default())
    }
}
