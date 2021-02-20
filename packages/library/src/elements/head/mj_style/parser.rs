use super::MJStyle;
use crate::elements::error::Error;
use crate::parser::{MJMLParser, Node};

#[derive(Default)]
struct MJStyleParser {
    inline: bool,
    content: Vec<String>,
}

impl MJMLParser for MJStyleParser {
    type Output = MJStyle;

    fn build(self) -> Result<Self::Output, Error> {
        Ok(MJStyle {
            inline: self.inline,
            content: self.content.join(""),
        })
    }

    fn parse(mut self, node: &Node) -> Result<Self, Error> {
        self.inline = node
            .attributes
            .iter()
            .filter(|(key, _value)| key.as_str() == "inline")
            .next()
            .is_some();
        self.content = node
            .children
            .iter()
            .filter_map(|item| item.as_text())
            .map(|item| item.to_string())
            .collect();

        Ok(self)
    }
}

impl MJStyle {
    pub fn parse(node: &Node) -> Result<Self, Error> {
        MJStyleParser::default().parse(node)?.build()
    }
}
