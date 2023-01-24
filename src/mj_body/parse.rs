use super::MjBodyChild;
use crate::mj_accordion::MjAccordion;
use crate::mj_accordion::NAME as MJ_ACCORDION;
use crate::mj_button::MjButton;
use crate::mj_button::NAME as MJ_BUTTON;
use crate::mj_carousel::MjCarousel;
use crate::mj_carousel::NAME as MJ_CAROUSEL;
use crate::mj_column::MjColumn;
use crate::mj_column::NAME as MJ_COLUMN;
use crate::mj_divider::MjDivider;
use crate::mj_divider::NAME as MJ_DIVIDER;
use crate::mj_group::MjGroup;
use crate::mj_group::NAME as MJ_GROUP;
use crate::mj_hero::MjHero;
use crate::mj_hero::NAME as MJ_HERO;
use crate::mj_image::MjImage;
use crate::mj_image::NAME as MJ_IMAGE;
use crate::mj_navbar::MjNavbar;
use crate::mj_navbar::NAME as MJ_NAVBAR;
use crate::mj_raw::MjRaw;
use crate::mj_raw::NAME as MJ_RAW;
use crate::mj_section::MjSection;
use crate::mj_section::NAME as MJ_SECTION;
use crate::mj_social::MjSocial;
use crate::mj_social::NAME as MJ_SOCIAL;
use crate::mj_spacer::MjSpacer;
use crate::mj_spacer::NAME as MJ_SPACER;
use crate::mj_table::MjTable;
use crate::mj_table::NAME as MJ_TABLE;
use crate::mj_text::MjText;
use crate::mj_text::NAME as MJ_TEXT;
use crate::mj_wrapper::MjWrapper;
use crate::mj_wrapper::NAME as MJ_WRAPPER;
use crate::node::Node;
use crate::prelude::parse::{Error, Parsable};
use xmlparser::{StrSpan, Tokenizer};

impl Parsable for MjBodyChild {
    fn parse<'a>(tag: StrSpan<'a>, tokenizer: &mut Tokenizer<'a>) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjAccordion::parse(tag, tokenizer)?.into()),
            MJ_BUTTON => Ok(MjButton::parse(tag, tokenizer)?.into()),
            MJ_CAROUSEL => Ok(MjCarousel::parse(tag, tokenizer)?.into()),
            MJ_COLUMN => Ok(MjColumn::parse(tag, tokenizer)?.into()),
            MJ_DIVIDER => Ok(MjDivider::parse(tag, tokenizer)?.into()),
            MJ_GROUP => Ok(MjGroup::parse(tag, tokenizer)?.into()),
            MJ_HERO => Ok(MjHero::parse(tag, tokenizer)?.into()),
            MJ_IMAGE => Ok(MjImage::parse(tag, tokenizer)?.into()),
            MJ_NAVBAR => Ok(MjNavbar::parse(tag, tokenizer)?.into()),
            MJ_RAW => Ok(MjRaw::parse(tag, tokenizer)?.into()),
            MJ_SECTION => Ok(MjSection::parse(tag, tokenizer)?.into()),
            MJ_SOCIAL => Ok(MjSocial::parse(tag, tokenizer)?.into()),
            MJ_SPACER => Ok(MjSpacer::parse(tag, tokenizer)?.into()),
            MJ_TABLE => Ok(MjTable::parse(tag, tokenizer)?.into()),
            MJ_TEXT => Ok(MjText::parse(tag, tokenizer)?.into()),
            MJ_WRAPPER => Ok(MjWrapper::parse(tag, tokenizer)?.into()),
            _ => Ok(Node::<MjBodyChild>::parse(tag, tokenizer)?.into()),
            // _ => Err(ParserError::UnexpectedElement(tag.start())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

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
        let elt = Mjml::parse(template).unwrap();
        assert!(elt.head().is_none());
        assert!(elt.body().is_some());
        let body = elt.body().unwrap();
        assert_eq!(body.children.len(), 2);
    }
}
