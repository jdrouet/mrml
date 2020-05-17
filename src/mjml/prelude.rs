use super::error::Error;
use crate::util::{Context, Header, Size, Spacing, Style};
use regex::Regex;
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

pub trait ComponentWithSizeAttribute: Component {
    fn get_size_attribute(&self, name: &str) -> Option<Size> {
        self.get_attribute(name)
            .and_then(|value| value.parse::<Size>().ok())
    }
}

pub trait ComponentWithBorder: Component {
    fn get_border_by_name(&self, name: &str) -> Option<Size> {
        self.get_attribute(name)
            .and_then(|border| {
                let re = Regex::new(r"(\d+)").unwrap();
                re.captures(border.as_str())
                    .and_then(|list| list.get(1))
                    .and_then(|first| first.as_str().parse::<f32>().ok())
            })
            .and_then(|value| Some(Size::Pixel(value)))
    }

    fn get_border_top(&self) -> Option<Size> {
        self.get_border_by_name("border-top")
            .or_else(|| self.get_border_by_name("border"))
    }

    fn get_border_bottom(&self) -> Option<Size> {
        self.get_border_by_name("border-botttom")
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
}

pub trait ComponentWithPadding: Component + ComponentWithSizeAttribute {
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
