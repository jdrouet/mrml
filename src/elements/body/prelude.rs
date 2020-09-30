use super::BodyElement;
use crate::elements::prelude::*;
use crate::util::attributes::Attributes;
use crate::util::size::Size;
use crate::util::spacing::Spacing;
use crate::util::tag::Tag;
use regex::Regex;

lazy_static! {
    pub static ref EMPTY_CHILDREN: Vec<BodyElement> = vec![];
}

pub trait BodyComponent: Component {
    fn attributes(&self) -> Option<&Attributes>;

    fn get_attribute(&self, key: &str) -> Option<&String> {
        self.attributes().and_then(|src| src.get(&key.to_string()))
    }
    fn get_size_attribute(&self, name: &str) -> Option<Size> {
        self.get_attribute(name)
            .and_then(|value| value.parse::<Size>().ok())
    }
    fn get_children(&self) -> &Vec<BodyElement>;
    fn get_current_width(&self) -> Option<Size>;

    fn get_siblings(&self) -> usize {
        self.get_children().len()
    }

    fn get_raw_siblings(&self) -> usize {
        self.get_children()
            .iter()
            .fold(0, |res, item| if item.is_raw() { res + 1 } else { res })
    }

    fn set_style(&self, _key: &str, tag: Tag) -> Tag {
        tag
    }

    fn get_width(&self) -> Option<Size> {
        None
    }

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
    use crate::elements::error::Error;
    use crate::util::attributes::Attributes;
    use crate::util::context::Context;
    use crate::util::header::Header;
    use crate::util::tag::Tag;

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

    impl BodyComponent for TestComponent {
        fn get_current_width(&self) -> Option<Size> {
            None
        }
        fn get_children(&self) -> &Vec<BodyElement> {
            &EMPTY_CHILDREN
        }
        fn attributes(&self) -> Option<&Attributes> {
            Some(&self.attributes)
        }
    }

    #[test]
    fn basic_component_default_values() {
        let item = TestComponent {
            attributes: Attributes::new(),
        };
        assert_eq!(item.get_attribute("nothing"), None);
        assert_eq!(item.set_style("nothing", Tag::new("a")).open(), "<a>");
    }

    #[test]
    fn component_with_border_default_values() {
        let attributes = Attributes::new()
            .add("border-top", "1px solid red")
            .add("border-bottom", "2px solid blue")
            .add("toto-border-top", "3px solid red")
            .add("toto-border-bottom", "4px solid blue");
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
        let attributes = Attributes::new()
            .add("border", "1px solid red")
            .add("toto-border", "2px solid red");
        let item = TestComponent { attributes };
        assert_eq!(item.get_border_top(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_border_bottom(), Some(Size::Pixel(1.0)));
        assert_eq!(item.get_prefixed_border_top("toto"), Some(Size::Pixel(2.0)));
        assert_eq!(
            item.get_prefixed_border_bottom("toto"),
            Some(Size::Pixel(2.0))
        );
    }

    #[test]
    fn component_with_size_attribute() {
        let attributes = Attributes::new()
            .add("padding-top", "1px")
            .add("padding-bottom", "something");
        let item = TestComponent { attributes };
        assert_eq!(
            item.get_size_attribute("padding-top"),
            Some(Size::Pixel(1.0))
        );
        assert_eq!(item.get_size_attribute("padding-bottom"), None);
    }

    #[test]
    fn component_with_padding_normal() {
        let attributes = Attributes::new()
            .add("padding-top", "1px")
            .add("padding-bottom", "2px")
            .add("toto-padding-top", "3px")
            .add("toto-padding-bottom", "4px");
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
        let attributes = Attributes::new()
            .add("padding", "1px")
            .add("toto-padding", "2px");
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

    #[test]
    fn component_with_box_widths() {
        let attributes = Attributes::new()
            .add("border-left", "2% solid blue")
            .add("border-right", "2px solid blue")
            .add("padding-left", "2%")
            .add("padding-right", "2px");
        let item = TestComponent { attributes };
        assert_eq!(item.get_border_horizontal_width(), Size::Pixel(0.0));
        assert_eq!(item.get_padding_horizontal_width(), Size::Pixel(0.0));
    }
}
