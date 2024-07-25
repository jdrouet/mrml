#[cfg(feature = "json")]
mod json;
#[cfg(feature = "render")]
mod network;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::mj_raw::MjRawChild;
use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-social-element";

pub struct MjSocialElementTag;

impl StaticTag for MjSocialElementTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjSocialElement =
    Component<PhantomData<MjSocialElementTag>, Map<String, String>, Vec<MjRawChild>>;
