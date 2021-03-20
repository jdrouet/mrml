use crate::mj_body::MJBodyChild;
use std::collections::HashMap;

mod print;

pub const NAME: &str = "mj-button";

#[derive(Debug, Default)]
pub struct MJButton {
    attributes: HashMap<String, String>,
    children: Vec<MJBodyChild>,
}
