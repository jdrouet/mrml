use crate::mjml::prelude::*;
use crate::util::attributes::*;
use crate::util::{Header, Size, Spacing, Tag};
use regex::Regex;
use roxmltree::Node;

pub trait ParsedBodyComponent {
    fn default_attributes() -> Attributes {
        Attributes::new()
    }

    fn create_attributes<'a, 'b>(node: &Node<'a, 'b>, header: &Header) -> Attributes {
        header
            .default_attributes()
            .get_attributes(node, Self::default_attributes())
    }
}

pub trait BodyComponent: Component {
    fn set_style(&self, _key: &str, tag: Tag) -> Tag {
        tag
    }

    fn get_width(&self) -> Option<Size> {
        None
    }
}

pub trait BodyComponentWithBorder: ComponentWithAttributes {
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

pub trait BodyComponentWithPadding: Component + ComponentWithSizeAttribute {
    fn get_padding_global_by_name(&self, name: &str) -> Option<Spacing> {
        self.get_attribute(name)
            .and_then(|value| value.parse::<Spacing>().ok())
    }

    fn get_prefixed_padding_global(&self, prefix: &str) -> Option<Spacing> {
        self.get_padding_global_by_name(format!("{}-padding", prefix).as_str())
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
        self.get_padding_global_by_name("padding")
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

    fn get_padding_vertical(&self) -> Option<Size> {
        self.get_padding_top().and_then(|top| {
            self.get_padding_bottom().and_then(|bot| {
                if top.is_pixel() && bot.is_pixel() {
                    Some(Size::Pixel(top.value() + bot.value()))
                } else if top.is_percent() && bot.is_percent() {
                    Some(Size::Percent(top.value() + bot.value()))
                } else {
                    None
                }
            })
        })
    }
}

pub trait BodyContainedComponent: Component {
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

pub trait BodyComponentWithBoxWidths:
    BodyComponentWithBorder + BodyComponentWithPadding + BodyContainedComponent
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

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::mjml::error::Error;
    use crate::util::{Context, Header, Tag};
    use std::collections::HashMap;

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

        fn render(&self, _header: &Header) -> Result<String, Error> {
            Ok("nothing".into())
        }
    }

    impl ComponentWithAttributes for TestComponent {
        fn attributes(&self) -> Option<&HashMap<String, String>> {
            Some(&self.attributes)
        }
    }

    impl BodyComponent for TestComponent {}

    #[test]
    fn basic_component_default_values() {
        let item = TestComponent {
            attributes: HashMap::new(),
        };
        assert_eq!(item.get_attribute("nothing"), None);
        assert_eq!(item.set_style("nothing", Tag::new("a")).open(), "<a>");
    }

    impl BodyComponentWithBorder for TestComponent {}

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

    impl BodyComponentWithPadding for TestComponent {}

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

    impl BodyContainedComponent for TestComponent {}
    impl BodyComponentWithBoxWidths for TestComponent {}

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
