use super::error::Error;
use crate::util::Properties;
use roxmltree::Node;

pub trait Component {
    fn allowed_attributes() -> Option<Vec<&'static str>> {
        None
    }

    fn default_attribute(key: &str) -> Option<String>;

    fn node(&self) -> Option<Node>;

    fn get_attribute(&self, key: &str) -> Option<String> {
        if let Some(allowed) = Self::allowed_attributes() {
            if !allowed.contains(&key) {
                return None;
            }
        }
        self.node()
            .and_then(|node| node.attribute(key).and_then(|v| Some(v.to_string())))
            .or_else(|| Self::default_attribute(key))
    }

    fn set_context(&mut self, ctx: Properties);

    fn render(&self) -> Result<String, Error>;
}
