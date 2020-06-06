pub const START_CONDITIONAL_TAG: &str = "<!--[if mso | IE]>";
pub const START_MSO_CONDITIONAL_TAG: &str = "<!--[if mso]>";
pub const END_CONDITIONAL_TAG: &str = "<![endif]-->";
pub const START_NEGATION_CONDITIONAL_TAG: &str = "<!--[if !mso | IE]><!-->";
pub const START_MSO_NEGATION_CONDITIONAL_TAG: &str = "<!--[if !mso><!-->";
pub const END_NEGATION_CONDITIONAL_TAG: &str = "<!--<![endif]-->";

pub fn conditional_tag(input: String) -> String {
    format!("{}{}{}", START_CONDITIONAL_TAG, input, END_CONDITIONAL_TAG)
}
