use super::body::BodyElement;
use super::error::Error;
use crate::util::{Context, Header, Size};
use roxmltree::Node;
use std::collections::HashMap;
use std::string::ToString;

pub fn get_node_attributes<'a, 'b>(node: &Node<'a, 'b>) -> HashMap<String, String> {
    let mut res = HashMap::<String, String>::new();
    for item in node.attributes().iter() {
        res.insert(item.name().to_string(), item.value().to_string());
    }
    res
}

pub trait Component {
    fn context(&self) -> Option<&Context>;
    fn set_context(&mut self, ctx: Context);

    fn to_header(&self) -> Header {
        Header::new()
    }

    fn render(&self) -> Result<String, Error>;
}

pub trait ComponentWithAttributes: Component {
    fn default_attribute(&self, _key: &str) -> Option<String> {
        None
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>>;

    fn get_source_attribute(&self, key: &str) -> Option<String> {
        self.source_attributes()
            .and_then(|src| src.get(&key.to_string()))
            .and_then(|value| Some(value.clone()))
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        self.get_source_attribute(key)
            .or_else(|| self.default_attribute(key))
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
        attributes: HashMap<String, String>,
    }

    impl Component for TestComponent {
        fn context(&self) -> Option<&Context> {
            None
        }

        fn set_context(&mut self, _ctx: Context) {
            // noop
        }

        fn render(&self) -> Result<String, Error> {
            Ok("nothing".into())
        }
    }

    impl ComponentWithAttributes for TestComponent {
        fn source_attributes(&self) -> Option<&HashMap<String, String>> {
            Some(&self.attributes)
        }
    }

    #[test]
    fn basic_component_default_values() {
        let mut item = TestComponent {
            attributes: HashMap::new(),
        };
        assert_eq!(item.context().is_none(), true);
        item.set_context(Context::default(Options::default()));
        assert_eq!(item.source_attributes(), Some(&item.attributes));
        assert_eq!(item.default_attribute("nothing"), None);
        assert_eq!(item.render().unwrap(), "nothing");
        let header = item.to_header();
        assert_eq!(header.get_font_families().len(), 0);
        assert_eq!(header.get_media_queries().is_empty(), true);
        assert_eq!(header.get_styles().is_empty(), true);
    }
}
