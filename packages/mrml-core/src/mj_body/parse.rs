use std::rc::Rc;

use xmlparser::{StrSpan, Tokenizer};

use super::MjBodyChild;
use crate::mj_accordion::{MjAccordion, NAME as MJ_ACCORDION};
use crate::mj_button::{MjButton, NAME as MJ_BUTTON};
use crate::mj_carousel::{MjCarousel, NAME as MJ_CAROUSEL};
use crate::mj_column::{MjColumn, NAME as MJ_COLUMN};
use crate::mj_divider::{MjDivider, NAME as MJ_DIVIDER};
use crate::mj_group::{MjGroup, NAME as MJ_GROUP};
use crate::mj_hero::{MjHero, NAME as MJ_HERO};
use crate::mj_image::{MjImage, NAME as MJ_IMAGE};
use crate::mj_include::body::MjIncludeBody;
use crate::mj_include::NAME as MJ_INCLUDE;
use crate::mj_navbar::{MjNavbar, NAME as MJ_NAVBAR};
use crate::mj_raw::{MjRaw, NAME as MJ_RAW};
use crate::mj_section::{MjSection, NAME as MJ_SECTION};
use crate::mj_social::{MjSocial, NAME as MJ_SOCIAL};
use crate::mj_spacer::{MjSpacer, NAME as MJ_SPACER};
use crate::mj_table::{MjTable, NAME as MJ_TABLE};
use crate::mj_text::{MjText, NAME as MJ_TEXT};
use crate::mj_wrapper::{MjWrapper, NAME as MJ_WRAPPER};
use crate::node::Node;
use crate::prelude::parser::{Error, Parsable, ParserOptions};

impl Parsable for MjBodyChild {
    fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        opts: Rc<ParserOptions>,
    ) -> Result<Self, Error> {
        match tag.as_str() {
            MJ_ACCORDION => Ok(MjAccordion::parse(tag, tokenizer, opts)?.into()),
            MJ_BUTTON => Ok(MjButton::parse(tag, tokenizer, opts)?.into()),
            MJ_CAROUSEL => Ok(MjCarousel::parse(tag, tokenizer, opts)?.into()),
            MJ_COLUMN => Ok(MjColumn::parse(tag, tokenizer, opts)?.into()),
            MJ_DIVIDER => Ok(MjDivider::parse(tag, tokenizer, opts)?.into()),
            MJ_GROUP => Ok(MjGroup::parse(tag, tokenizer, opts)?.into()),
            MJ_HERO => Ok(MjHero::parse(tag, tokenizer, opts)?.into()),
            MJ_IMAGE => Ok(MjImage::parse(tag, tokenizer, opts)?.into()),
            MJ_INCLUDE => Ok(MjIncludeBody::parse(tag, tokenizer, opts)?.into()),
            MJ_NAVBAR => Ok(MjNavbar::parse(tag, tokenizer, opts)?.into()),
            MJ_RAW => Ok(MjRaw::parse(tag, tokenizer, opts)?.into()),
            MJ_SECTION => Ok(MjSection::parse(tag, tokenizer, opts)?.into()),
            MJ_SOCIAL => Ok(MjSocial::parse(tag, tokenizer, opts)?.into()),
            MJ_SPACER => Ok(MjSpacer::parse(tag, tokenizer, opts)?.into()),
            MJ_TABLE => Ok(MjTable::parse(tag, tokenizer, opts)?.into()),
            MJ_TEXT => Ok(MjText::parse(tag, tokenizer, opts)?.into()),
            MJ_WRAPPER => Ok(MjWrapper::parse(tag, tokenizer, opts)?.into()),
            _ => Ok(Node::<MjBodyChild>::parse(tag, tokenizer, opts)?.into()),
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
