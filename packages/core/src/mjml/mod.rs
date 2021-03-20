use crate::mj_body::MJBody;
use crate::mj_head::MJHead;

mod print;

pub const NAME: &str = "mjml";

#[derive(Debug, Default)]
pub struct MJML {
    head: Option<MJHead>,
    body: Option<MJBody>,
}
