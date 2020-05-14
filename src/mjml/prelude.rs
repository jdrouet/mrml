use super::error::Error;
use crate::util::{Context, Header, Size, Style};
use roxmltree::Node;
use std::string::ToString;

pub trait Component {
    fn allowed_attributes() -> Option<Vec<&'static str>> {
        None
    }

    fn default_attribute(_key: &str) -> Option<String> {
        None
    }

    fn node(&self) -> Option<Node>;

    fn get_style(&self, _key: &str) -> Style {
        Style::new()
    }

    fn context(&self) -> Option<&Context>;

    fn to_header(&self) -> Header {
        Header::new()
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        if let Some(allowed) = Self::allowed_attributes() {
            if !allowed.contains(&key) {
                return None;
            }
        }
        self.node()
            .and_then(|node| node.attribute(key))
            .and_then(|value| Some(value.to_string()))
            .or_else(|| Self::default_attribute(key))
    }

    fn set_context(&mut self, ctx: Context);

    fn render(&self) -> Result<String, Error>;
}

pub trait ContainedComponent: Component {
    fn get_container_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn get_container_width_str(&self) -> Option<String> {
        self.get_container_width()
            .and_then(|width| Some(width.to_string()))
    }

    fn get_container_width_value(&self) -> Option<usize> {
        self.get_container_width()
            .and_then(|width| Some(width.value()))
    }
}
