use super::MJTitle;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};

#[derive(Default)]
struct MJTitleParser {
    content: Option<String>,
}

impl MJMLParser for MJTitleParser {
    type Output = MJTitle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJTitle {
            content: self.content.unwrap_or_default(),
        })
    }

    fn parse(mut self, node: &Node) -> Result<Self, Error> {
        self.content = node
            .children
            .iter()
            .filter_map(|item| item.as_text())
            .map(|item| item.to_string())
            .next();

        Ok(self)
    }
}

impl MJTitle {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJTitleParser::default().parse(node)?.build()
    }
}
