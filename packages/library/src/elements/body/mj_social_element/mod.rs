mod parser;
mod renderer;

use crate::util::attributes::*;
use crate::util::context::Context;

pub const NAME: &str = "mj-social-element";

#[derive(Debug)]
struct SocialNetwork {
    pub background_color: String,
    pub share_url: Option<String>,
    pub src: String,
}

#[derive(Debug)]
pub struct MJSocialElement {
    attributes: Attributes,
    context: Option<Context>,
    content: Option<String>,
    social_network: Option<SocialNetwork>,
}
