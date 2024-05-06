use crate::comment::Comment;
use crate::mj_attributes::MjAttributes;
use crate::mj_breakpoint::MjBreakpoint;
use crate::mj_font::MjFont;
use crate::mj_include::head::MjIncludeHead;
use crate::mj_preview::MjPreview;
use crate::mj_raw::MjRaw;
use crate::mj_style::MjStyle;
use crate::mj_title::MjTitle;

#[derive(Debug)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", enum_dispatch::enum_dispatch)]
#[cfg_attr(feature = "render", derive(enum_as_inner::EnumAsInner))]
pub enum MjHeadChild {
    Comment(Comment),
    MjAttributes(MjAttributes),
    MjBreakpoint(MjBreakpoint),
    MjFont(MjFont),
    MjInclude(MjIncludeHead),
    MjPreview(MjPreview),
    MjRaw(MjRaw),
    MjStyle(MjStyle),
    MjTitle(MjTitle),
}
