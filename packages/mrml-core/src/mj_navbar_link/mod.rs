#[cfg(feature = "json")]
mod json;
#[cfg(feature = "parse")]
mod parse;
#[cfg(feature = "print")]
mod print;
#[cfg(feature = "render")]
mod render;

use std::marker::PhantomData;

use crate::mj_raw::MjRawChild;
use crate::prelude::{Component, StaticTag};

pub const NAME: &str = "mj-navbar-link";

pub struct MjNavbarLinkTag;

impl StaticTag for MjNavbarLinkTag {
    fn static_tag() -> &'static str {
        NAME
    }
}

pub type MjNavbarLink =
    Component<PhantomData<MjNavbarLinkTag>, crate::prelude::AttributeMap, Vec<MjRawChild>>;
