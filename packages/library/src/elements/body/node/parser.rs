use super::Node;
use crate::elements::body::comment::Comment;
use crate::elements::body::raw::RawElement;
use crate::elements::body::text::Text;
use crate::elements::body::BodyElement;
use crate::elements::Error;
use crate::parser::MJMLParser;
use crate::util::attributes::Attributes;
use crate::util::header::Header;
use xmlparser::{StrSpan, Tokenizer};

struct NodeParser<'h> {
    header: &'h Header,
    only_raw: bool,
    name: String,
    attributes: Attributes,
    children: Vec<BodyElement>,
}

impl<'h> NodeParser<'h> {
    pub fn new<'a>(tag: StrSpan<'a>, header: &'h Header, only_raw: bool) -> Self {
        Self {
            header,
            only_raw,
            name: tag.to_string(),
            attributes: Attributes::default(),
            children: Vec::new(),
        }
    }
}

impl<'h> MJMLParser for NodeParser<'h> {
    type Output = Node;

    fn build(self) -> Result<Self::Output, Error> {
        if self.only_raw && self.name.starts_with("mj-") {
            return Err(Error::UnexpectedElement(self.name));
        }
        Ok(Node {
            name: self.name,
            attributes: self.attributes,
            context: None,
            children: self.children,
        })
    }

    fn parse_attribute<'a>(&mut self, name: StrSpan<'a>, value: StrSpan<'a>) -> Result<(), Error> {
        self.attributes.set(name, value);
        Ok(())
    }

    fn parse_child_comment(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(Comment::from(value.to_string()).into());
        Ok(())
    }

    fn parse_child_text(&mut self, value: StrSpan) -> Result<(), Error> {
        self.children.push(Text::from(value.to_string()).into());
        Ok(())
    }

    fn parse_child_element<'a>(
        &mut self,
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
    ) -> Result<(), Error> {
        self.children.push(if self.only_raw {
            RawElement::conditional_parse(tag, tokenizer, self.header, true)?.into()
        } else {
            BodyElement::parse(tag, tokenizer, self.header, None)?
        });
        Ok(())
    }
}

impl Node {
    pub fn parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
    ) -> Result<Node, Error> {
        Node::conditional_parse(tag, tokenizer, header, false)
    }

    pub fn conditional_parse<'a>(
        tag: StrSpan<'a>,
        tokenizer: &mut Tokenizer<'a>,
        header: &Header,
        only_raw: bool,
    ) -> Result<Node, Error> {
        NodeParser::new(tag, header, only_raw)
            .parse(tokenizer)?
            .build()
    }
}
