use crate::comment::Comment;
use crate::mj_attributes::MJAttributes;
use crate::mj_breakpoint::MJBreakpoint;
use crate::mj_font::MJFont;
use crate::mj_preview::MJPreview;
use crate::mj_raw::MJRaw;
use crate::mj_style::MJStyle;
use crate::mj_title::MJTitle;

#[derive(Debug, mrml_macros::MrmlChildren)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "json", serde(untagged))]
#[cfg_attr(feature = "print", derive(mrml_print_macros::MrmlPrintChildren))]
#[cfg_attr(feature = "parse", derive(mrml_parse_macros::MrmlParseChildren))]
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
