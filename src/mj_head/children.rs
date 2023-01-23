use crate::comment::Comment;
use crate::mj_attributes::MjAttributes;
use crate::mj_breakpoint::MjBreakpoint;
use crate::mj_font::MjFont;
use crate::mj_preview::MjPreview;
use crate::mj_raw::MjRaw;
use crate::mj_style::MjStyle;
use crate::mj_title::MjTitle;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseChildren))]
pub enum MjHeadChild {
    Comment(Comment),
    MjAttributes(MjAttributes),
    MjBreakpoint(MjBreakpoint),
    MjFont(MjFont),
    MjPreview(MjPreview),
    MjRaw(MjRaw),
    MjStyle(MjStyle),
    MjTitle(MjTitle),
}
