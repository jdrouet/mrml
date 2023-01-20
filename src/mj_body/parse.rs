use super::MJBodyChild;
use crate::mj_accordion::MJAccordion;
use crate::mj_accordion::NAME as MJ_ACCORDION;
use crate::mj_button::MJButton;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_carousel::MJCarousel;
use crate::mj_carousel::NAME as MJ_CAROUSEL;
use crate::mj_column::MJColumn;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::MJDivider;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_group::MJGroup;
use crate::mj_group::NAME as MJ_GROUP;
use crate::mj_hero::MJHero;
use crate::mj_hero::NAME as MJ_HERO;
use crate::mj_image::MJImage;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_navbar::MJNavbar;
use crate::mj_navbar::NAME as MJ_NAVBAR;
use crate::mj_raw::MJRaw;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::MJSection;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_social::MJSocial;
use crate::mj_social::NAME as MJ_SOCIAL;
use crate::mj_spacer::MJSpacer;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_table::MJTable;
use crate::mj_table::NAME as MJ_TABLE;
use crate::mj_text::MJText;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::MJWrapper;
use crate::mj_wrapper::NAME as MJ_WRAPPER;
use crate::node::Node;
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MJBodyChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MJAccordion::parse(tag, tokenizer)?.into()),
            MJ_BUTTON => Ok(MJButton::parse(tag, tokenizer)?.into()),
            MJ_CAROUSEL => Ok(MJCarousel::parse(tag, tokenizer)?.into()),
            MJ_COLUMN => Ok(MJColumn::parse(tag, tokenizer)?.into()),
            MJ_DIVIDER => Ok(MJDivider::parse(tag, tokenizer)?.into()),
            MJ_GROUP => Ok(MJGroup::parse(tag, tokenizer)?.into()),
            MJ_HERO => Ok(MJHero::parse(tag, tokenizer)?.into()),
            MJ_IMAGE => Ok(MJImage::parse(tag, tokenizer)?.into()),
            MJ_NAVBAR => Ok(MJNavbar::parse(tag, tokenizer)?.into()),
            MJ_RAW => Ok(MJRaw::parse(tag, tokenizer)?.into()),
            MJ_SECTION => Ok(MJSection::parse(tag, tokenizer)?.into()),
            MJ_SOCIAL => Ok(MJSocial::parse(tag, tokenizer)?.into()),
            MJ_SPACER => Ok(MJSpacer::parse(tag, tokenizer)?.into()),
            MJ_TABLE => Ok(MJTable::parse(tag, tokenizer)?.into()),
            MJ_TEXT => Ok(MJText::parse(tag, tokenizer)?.into()),
            MJ_WRAPPER => Ok(MJWrapper::parse(tag, tokenizer)?.into()),
            _ => Ok(Node::<MJBodyChild>::parse(tag, tokenizer)?.into()),
            // _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

    #[test]
    fn parse_complete() {
        let template = r#"
        <mjml>
            <mj-body>
                <!-- Some comment -->
                <mj-button>Hello World</mj-button>
            </mj-body>
        </mjml>
        "#;
        let elt = MJML::parse(template).unwrap();
        assert!(elt.head().is_none());
        assert!(elt.body().is_some());
        let body = elt.body().unwrap();
        assert_eq!(body.children.len(), 2);
    }
}
