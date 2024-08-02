use std::marker::PhantomData;

use crate::mj_body::MjBodyChild;
use crate::prelude::hash::Map;
use crate::prelude::{Component, StaticTag};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

pub const NAME: &str = "mj-group";

pub struct MjGroupTag;

impl StaticTag for MjGroupTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjGroup = Component<PhantomData<MjGroupTag>, Map<String, String>, Vec<MjBodyChild>>;
