use super::body::BodyElement;
use super::error::Error;
use crate::parser::Node;
use crate::util::attributes::Attributes;
use crate::util::{Context, Header, Size};
use std::collections::HashMap;
use std::string::ToString;

pub fn get_node_attributes<'a>(node: &Node<'a>) -> HashMap<String, String> {
    let mut res = HashMap::<String, String>::new();
    add_node_attributes(&mut res, node);
    res
}

pub fn add_node_attributes<'a>(res: &mut HashMap<String, String>, node: &Node<'a>) {
    for (key, value) in node.attributes.iter() {
        res.insert(key.as_str().into(), value.as_str().into());
    }
}

pub trait Component {
    fn context(&self) -> Option<&Context>;
    fn set_context(&mut self, ctx: Context);

    fn update_header(&self, _header: &mut Header) {}
    fn render(&self, header: &Header) -> Result<String, Error>;
}

pub trait ComponentWithAttributes: Component {
    fn attributes(&self) -> Option<&Attributes>;

    fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes().and_then(|src| src.get(&key.to_string()))
    }
}

pub trait ComponentWithSizeAttribute: ComponentWithAttributes {
    fn get_size_attribute(&self, name: &str) -> Option<Size> {
        self.get_attribute(name)
            .and_then(|value| value.parse::<Size>().ok())
    }
}

pub trait ComponentWithChildren: Component {
    fn get_children(&self) -> &Vec<BodyElement>;
    fn get_current_width(&self) -> Option<Size>;

    fn get_siblings(&self) -> usize {
        self.get_children().len()
    }

    fn get_raw_siblings(&self) -> usize {
        self.get_children().iter().fold(0, |res, item| match item {
            BodyElement::Raw(_) => res + 1,
            _ => res,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::Options;

    struct TestComponent {
        attributes: Attributes,
    }

    impl Component for TestComponent {
        fn context(&self) -> Option<&Context> {
            None
        }

        fn set_context(&mut self, _ctx: Context) {
            // noop
        }

        fn render(&self, _header: &Header) -> Result<String, Error> {
            Ok("nothing".into())
        }
    }

    impl ComponentWithAttributes for TestComponent {
        fn attributes(&self) -> Option<&Attributes> {
            Some(&self.attributes)
        }
    }

    #[test]
    fn basic_component_default_values() {
        let header = Header::from(Options::default());
        let mut item = TestComponent {
            attributes: Attributes::new(),
        };
        assert_eq!(item.context().is_none(), true);
        item.set_context(Context::default());
        assert_eq!(item.get_attribute("nothing"), None);
        assert_eq!(item.render(&header).unwrap(), "nothing");
    }
}
