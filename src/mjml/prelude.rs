use super::error::Error;
use super::Element;
use crate::util::{Context, Header, Size, Spacing, Style};
use regex::Regex;
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
    fn allowed_attributes(&self) -> Option<Vec<&'static str>> {
        None
    }

    fn default_attribute(&self, _key: &str) -> Option<String> {
        None
    }

    fn get_style(&self, _key: &str) -> Style {
        Style::new()
    }

    fn context(&self) -> Option<&Context>;

    fn to_header(&self) -> Header {
        Header::new()
    }

    fn source_attributes(&self) -> Option<&HashMap<String, String>>;

    fn get_source_attribute(&self, key: &str) -> Option<String> {
        self.source_attributes()
            .and_then(|src| src.get(&key.to_string()))
            .and_then(|value| Some(value.clone()))
    }

    fn get_attribute(&self, key: &str) -> Option<String> {
        if let Some(allowed) = self.allowed_attributes() {
            if !allowed.contains(&key) {
                return None;
            }
        }
        self.get_source_attribute(key)
            .or_else(|| self.default_attribute(key))
    }

    fn set_context(&mut self, ctx: Context);

    fn render(&self) -> Result<String, Error>;
}

pub trait ComponentWithSizeAttribute: Component {
    fn get_size_attribute(&self, name: &str) -> Option<Size> {
        self.get_attribute(name)
            .and_then(|value| value.parse::<Size>().ok())
    }
}

pub trait ComponentWithBorder: Component {
    fn get_border_by_name(&self, name: &str) -> Option<Size> {
        self.get_attribute(name).and_then(|border| {
            let re = Regex::new(r"(\d+)").unwrap();
            re.captures(border.as_str())
                .and_then(|list| list.get(1))
                .and_then(|first| first.as_str().parse::<f32>().ok())
                .and_then(move |value| {
                    if border.contains("%") {
                        Some(Size::Percent(value))
                    } else {
                        Some(Size::Pixel(value))
                    }
                })
        })
    }

    fn get_border_top(&self) -> Option<Size> {
        self.get_border_by_name("border-top")
            .or_else(|| self.get_border_by_name("border"))
    }

    fn get_border_bottom(&self) -> Option<Size> {
        self.get_border_by_name("border-bottom")
            .or_else(|| self.get_border_by_name("border"))
    }

    fn get_border_left(&self) -> Option<Size> {
        self.get_border_by_name("border-left")
            .or_else(|| self.get_border_by_name("border"))
    }

    fn get_border_right(&self) -> Option<Size> {
        self.get_border_by_name("border-right")
            .or_else(|| self.get_border_by_name("border"))
    }

    fn get_prefixed_border_top(&self, prefix: &str) -> Option<Size> {
        self.get_border_by_name(format!("{}-border-top", prefix).as_str())
            .or_else(|| self.get_border_by_name(format!("{}-border", prefix).as_str()))
    }

    fn get_prefixed_border_bottom(&self, prefix: &str) -> Option<Size> {
        self.get_border_by_name(format!("{}-border-bottom", prefix).as_str())
            .or_else(|| self.get_border_by_name(format!("{}-border", prefix).as_str()))
    }

    fn get_prefixed_border_left(&self, prefix: &str) -> Option<Size> {
        self.get_border_by_name(format!("{}-border-left", prefix).as_str())
            .or_else(|| self.get_border_by_name(format!("{}-border", prefix).as_str()))
    }

    fn get_prefixed_border_right(&self, prefix: &str) -> Option<Size> {
        self.get_border_by_name(format!("{}-border-right", prefix).as_str())
            .or_else(|| self.get_border_by_name(format!("{}-border", prefix).as_str()))
    }
}

pub trait ComponentWithPadding: Component + ComponentWithSizeAttribute {
    fn get_prefixed_padding_global(&self, prefix: &str) -> Option<Spacing> {
        self.get_attribute(format!("{}-padding", prefix).as_str())
            .and_then(|value| value.parse::<Spacing>().ok())
    }

    fn get_prefixed_padding(&self, prefix: &str, direction: &str) -> Option<Size> {
        self.get_size_attribute(format!("{}-padding-{}", prefix, direction).as_str())
            .or_else(|| {
                self.get_prefixed_padding_global(prefix)
                    .and_then(|value| value.get(direction))
            })
    }

    fn get_prefixed_padding_top(&self, prefix: &str) -> Option<Size> {
        self.get_prefixed_padding(prefix, "top")
    }

    fn get_prefixed_padding_right(&self, prefix: &str) -> Option<Size> {
        self.get_prefixed_padding(prefix, "right")
    }

    fn get_prefixed_padding_bottom(&self, prefix: &str) -> Option<Size> {
        self.get_prefixed_padding(prefix, "bottom")
    }

    fn get_prefixed_padding_left(&self, prefix: &str) -> Option<Size> {
        self.get_prefixed_padding(prefix, "left")
    }

    fn get_padding_global(&self) -> Option<Spacing> {
        self.get_attribute("padding")
            .and_then(|value| value.parse::<Spacing>().ok())
    }

    fn get_padding(&self, direction: &str) -> Option<Size> {
        self.get_size_attribute(format!("padding-{}", direction).as_str())
            .or_else(|| {
                self.get_padding_global()
                    .and_then(|value| value.get(direction))
            })
    }

    fn get_padding_top(&self) -> Option<Size> {
        self.get_padding("top")
    }

    fn get_padding_bottom(&self) -> Option<Size> {
        self.get_padding("bottom")
    }

    fn get_padding_left(&self) -> Option<Size> {
        self.get_padding("left")
    }

    fn get_padding_right(&self) -> Option<Size> {
        self.get_padding("right")
    }
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
            .and_then(|width| Some(width.value() as usize))
    }
}

pub trait ComponentWithBoxWidths:
    ComponentWithBorder + ComponentWithPadding + ContainedComponent
{
    fn get_border_horizontal_width(&self) -> Size {
        let left = match self.get_border_left() {
            Some(value) => value,
            None => Size::Pixel(0.0),
        };
        let right = match self.get_border_right() {
            Some(value) => value,
            None => Size::Pixel(0.0),
        };
        if left.is_pixel() && right.is_pixel() {
            Size::Pixel(left.value() + right.value())
        } else {
            Size::Pixel(0.0)
        }
    }

    fn get_padding_horizontal_width(&self) -> Size {
        let left = match self.get_padding_left() {
            Some(value) => value,
            None => Size::Pixel(0.0),
        };
        let right = match self.get_padding_right() {
            Some(value) => value,
            None => Size::Pixel(0.0),
        };
        if left.is_pixel() && right.is_pixel() {
            Size::Pixel(left.value() + right.value())
        } else {
            Size::Pixel(0.0)
        }
    }

    fn get_box_widths(&self) -> Option<Size> {
        self.get_container_width().and_then(|width| {
            let paddings = self.get_padding_horizontal_width();
            let borders = self.get_border_horizontal_width();
            Some(Size::Pixel(
                width.value() - paddings.value() - borders.value(),
            ))
        })
    }
}

pub trait ComponentWithChildren: Component {
    fn get_children(&self) -> &Vec<Element>;
    fn get_current_width(&self) -> Option<Size>;

    fn get_siblings(&self) -> usize {
        self.get_children().len()
    }

    fn get_raw_siblings(&self) -> usize {
        self.get_children().iter().fold(0, |res, item| match item {
            Element::Raw(_) => res + 1,
            _ => res,
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::util::prelude::PropertyMap;

    struct TestComponent {
        attributes: HashMap<String, String>,
    }

    impl Component for TestComponent {
        fn context(&self) -> Option<&Context> {
            None
        }

        fn source_attributes(&self) -> Option<&HashMap<String, String>> {
            Some(&self.attributes)
        }

        fn set_context(&mut self, _ctx: Context) {
            // noop
        }

        fn render(&self) -> Result<String, Error> {
            Ok("nothing".into())
        }
    }

    #[test]
    fn basic_component_default_values() {
        let item = TestComponent {
            attributes: HashMap::new(),
        };
        assert_eq!(item.allowed_attributes(), None);
        assert_eq!(item.default_attribute("nothing"), None);
        assert_eq!(item.get_style("nothing").is_empty(), true);
        let header = item.to_header();
        assert_eq!(header.get_font_families().len(), 0);
        assert_eq!(header.get_media_queries().is_empty(), true);
        assert_eq!(header.get_styles().is_empty(), true);
    }

    impl ComponentWithBorder for TestComponent {}

    #[test]
    fn component_with_border_default_values() {
        let mut attributes = HashMap::new();
        attributes.insert("border-top".to_string(), "1px solid red".to_string());
        attributes.insert("border-bottom".to_string(), "2px solid blue".to_string());
        attributes.insert("toto-border-top".to_string(), "3px solid red".to_string());
        attributes.insert(
            "toto-border-bottom".to_string(),
            "4px solid blue".to_string(),
        );
        let item = TestComponent { attributes };
        assert_eq!(item.get_border_top(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_border_bottom(), Some(Size::Pixel(2.0)));
        assert_eq!(item.get_prefixed_border_top("toto"), Some(Size::Pixel(3.0)));
        assert_eq!(
            item.get_prefixed_border_bottom("toto"),
            Some(Size::Pixel(4.0))
        );
    }

    #[test]
    fn component_with_border_common_border() {
        let mut attributes = HashMap::new();
        attributes.insert("border".to_string(), "1px solid red".to_string());
        attributes.insert("toto-border".to_string(), "2px solid red".to_string());
        let item = TestComponent { attributes };
        assert_eq!(item.get_border_top(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_border_bottom(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_prefixed_border_top("toto"), Some(Size::Pixel(2.0)));
        assert_eq!(
            item.get_prefixed_border_bottom("toto"),
            Some(Size::Pixel(2.0))
        );
    }

    impl ComponentWithSizeAttribute for TestComponent {}

    #[test]
    fn component_with_size_attribute() {
        let mut attributes = HashMap::new();
        attributes.insert("padding-top".to_string(), "1px".to_string());
        attributes.insert("padding-bottom".to_string(), "something".to_string());
        let item = TestComponent { attributes };
        assert_eq!(
            item.get_size_attribute("padding-top"),
            Some(Size::Pixel(1.0))
        );
        assert_eq!(item.get_size_attribute("padding-bottom"), None);
    }

    impl ComponentWithPadding for TestComponent {}

    #[test]
    fn component_with_padding_normal() {
        let mut attributes = HashMap::new();
        attributes.insert("padding-top".to_string(), "1px".to_string());
        attributes.insert("padding-bottom".to_string(), "2px".to_string());
        attributes.insert("toto-padding-top".to_string(), "3px".to_string());
        attributes.insert("toto-padding-bottom".to_string(), "4px".to_string());
        let item = TestComponent { attributes };
        assert_eq!(item.get_padding_top(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_padding_bottom(), Some(Size::Pixel(2.0)));
        assert_eq!(
            item.get_prefixed_padding_top("toto"),
            Some(Size::Pixel(3.0))
        );
        assert_eq!(
            item.get_prefixed_padding_bottom("toto"),
            Some(Size::Pixel(4.0))
        );
    }

    #[test]
    fn component_with_padding_common() {
        let mut attributes = HashMap::new();
        attributes.insert("padding".to_string(), "1px".to_string());
        attributes.insert("toto-padding".to_string(), "2px".to_string());
        let item = TestComponent { attributes };
        assert_eq!(item.get_padding_top(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_padding_bottom(), Some(Size::Pixel(1.0)));
        assert_eq!(
            item.get_prefixed_padding_top("toto"),
            Some(Size::Pixel(2.0))
        );
        assert_eq!(
            item.get_prefixed_padding_bottom("toto"),
            Some(Size::Pixel(2.0))
        );
    }

    impl ContainedComponent for TestComponent {}
    impl ComponentWithBoxWidths for TestComponent {}

    #[test]
    fn component_with_box_widths() {
        let mut attributes = HashMap::new();
        attributes.insert("border-left".to_string(), "2% solid blue".to_string());
        attributes.insert("border-right".to_string(), "2px solid blue".to_string());
        attributes.insert("padding-left".to_string(), "2%".to_string());
        attributes.insert("padding-right".to_string(), "2px".to_string());
        let item = TestComponent { attributes };
        assert_eq!(item.get_border_horizontal_width(), Size::Pixel(0.0));
        assert_eq!(item.get_padding_horizontal_width(), Size::Pixel(0.0));
    }
}
