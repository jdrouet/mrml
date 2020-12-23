mod parser;
mod renderer;

use crate::util::header::DefaultAttributes;

#[derive(Clone, Debug)]
pub struct MJAttributes(DefaultAttributes);

impl MJAttributes {
    fn new() -> Self {
        Self(DefaultAttributes::default())
    }
}
