use super::MJPreview;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};

#[derive(Default)]
struct MJPreviewParser {
    content: Option<String>,
}

impl MJMLParser for MJPreviewParser {
    type Output = MJPreview;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJPreview {
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

impl MJPreview {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJPreviewParser::default().parse(node)?.build()
    }
}
