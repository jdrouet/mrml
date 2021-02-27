pub const START_CONDITIONAL_TAG: &str = "<!--[if mso | IE]>";
pub const START_MSO_CONDITIONAL_TAG: &str = "<!--[if mso]>";
pub const END_CONDITIONAL_TAG: &str = "<![endif]-->";
pub const START_NEGATION_CONDITIONAL_TAG: &str = "<!--[if !mso | IE]><!-->";
pub const START_MSO_NEGATION_CONDITIONAL_TAG: &str = "<!--[if !mso><!-->";
pub const END_NEGATION_CONDITIONAL_TAG: &str = "<!--<![endif]-->";

pub fn conditional_tag(input: String) -> String {
    START_CONDITIONAL_TAG.to_string() + &input + &END_CONDITIONAL_TAG
}

pub fn negation_conditional_tag(input: String) -> String {
    START_NEGATION_CONDITIONAL_TAG.to_string() + &input + &END_NEGATION_CONDITIONAL_TAG
}

pub fn mso_conditional_tag(input: String) -> String {
    START_MSO_CONDITIONAL_TAG.to_string() + &input + &END_CONDITIONAL_TAG
}

pub fn mso_negation_conditional_tag(input: String) -> String {
    START_MSO_NEGATION_CONDITIONAL_TAG.to_string() + &input + &END_NEGATION_CONDITIONAL_TAG
}
