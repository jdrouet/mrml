use super::MJHero;
use crate::mj_body::MJBodyChild;
use crate::prelude::parse::{Error, Parser};
use crate::{parse_attribute, parse_child, parse_comment, parse_text};
use xmlparser::Tokenizer;

#[derive(Debug, Default)]
struct MJHeroParser(MJHero);

impl Parser for MJHeroParser {
    type Output = MJHero;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(self.0)
    }

    parse_attribute!();
    parse_child!(MJBodyChild);
    parse_comment!();
    parse_text!();
}

impl MJHero {
    pub fn parse(tokenizer: &mut Tokenizer) -> Result<Self, Error> {
        MJHeroParser::default().parse(tokenizer)?.build()
    }
}
