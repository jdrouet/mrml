use crate::comment::Comment;
use crate::mj_attributes::MJAttributes;
use crate::mj_breakpoint::MJBreakpoint;
use crate::mj_font::MJFont;
use crate::mj_preview::MJPreview;
use crate::mj_raw::MJRaw;
use crate::mj_style::MJStyle;
use crate::mj_title::MJTitle;
use crate::{as_child, from_child};

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
pub enum MJHeadChild {
    Comment(Comment),
    MJAttributes(MJAttributes),
    MJBreakpoint(MJBreakpoint),
    MJFont(MJFont),
    MJPreview(MJPreview),
    MJRaw(MJRaw),
    MJStyle(MJStyle),
    MJTitle(MJTitle),
}

from_child!(MJHeadChild, Comment);
as_child!(MJHeadChild, MJAttributes, as_mj_attributes);
from_child!(MJHeadChild, MJAttributes);
as_child!(MJHeadChild, MJBreakpoint, as_mj_breakpoint);
from_child!(MJHeadChild, MJBreakpoint);
as_child!(MJHeadChild, MJFont, as_mj_font);
from_child!(MJHeadChild, MJFont);
as_child!(MJHeadChild, MJPreview, as_mj_preview);
from_child!(MJHeadChild, MJPreview);
as_child!(MJHeadChild, MJRaw, as_mj_raw);
from_child!(MJHeadChild, MJRaw);
as_child!(MJHeadChild, MJStyle, as_mj_style);
from_child!(MJHeadChild, MJStyle);
as_child!(MJHeadChild, MJTitle, as_mj_title);
from_child!(MJHeadChild, MJTitle);
