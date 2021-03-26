use super::{MJButton, NAME};
use crate::helper::buffer::Buffer;
use crate::helper::size::Pixel;
use crate::helper::spacing::Spacing;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

struct MJButtonRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJButton,
}

impl<'e, 'h> MJButtonRender<'e, 'h> {
    fn pixel_attribute(&self, name: &str) -> Option<Pixel> {
        self.attribute(name)
            .and_then(|value| Pixel::try_from(value.as_str()).ok())
    }

    fn spacing_attribute(&self, name: &str) -> Option<Spacing> {
        self.attribute(name)
            .and_then(|value| Spacing::try_from(value.as_str()).ok())
    }

    fn content_width(&self) -> Option<String> {
        if let Some(width) = self.pixel_attribute("width") {
            let pad_left = self
                .pixel_attribute("inner-padding-left")
                .map(|pad| pad.value())
                .or_else(|| {
                    self.spacing_attribute("inner-padding")
                        .map(|pad| pad.left().value())
                })
                .unwrap_or(0.0);
            let pad_right = self
                .pixel_attribute("inner-padding-right")
                .map(|pad| pad.value())
                .or_else(|| {
                    self.spacing_attribute("inner-padding")
                        .map(|pad| pad.right().value())
                })
                .unwrap_or(0.0);
            Some(Pixel::new(width.value() - pad_left - pad_right).to_string())
        } else {
            None
        }
    }
}

impl<'e, 'h> Render<'h> for MJButtonRender<'e, 'h> {
    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, buf: &mut Buffer) -> Result<(), Error> {
        if let Some(ref font_family) = self.attribute("font-family") {
            self.header.borrow_mut().add_used_font_family(font_family);
        }
        buf.push_str("<table role=\"presentation\" style=\"border-collapse:separate;");
        buf.push_optional_style("width", self.attribute("width"));
        buf.push_str("line-height:100%\">");
        buf.push_str("<tr>");
        buf.push_str("<td align=\"center\"");
        buf.push_optional_attribute("bgcolor", self.attribute("background-color"));
        buf.push_str(" role=\"presentation\"");
        buf.push_optional_attribute("valign", self.attribute("vertical-align"));
        buf.push_str(">");
        let href = self.attribute("href");
        if let Some(ref href) = href {
            buf.push_str("<a href=\"");
            buf.push_str(href);
            buf.push('"');
            buf.push_optional_attribute("target", self.attribute("target"));
        } else {
            buf.push_str("<p");
        }
        buf.push_optional_attribute("rel", self.attribute("rel"));
        buf.push_optional_attribute("name", self.attribute("name"));
        buf.push_str(" style=\"display:inline-block;");
        buf.push_optional_style("width", self.content_width());
        buf.push_optional_style("background", self.attribute("background-color"));
        buf.push_optional_style("color", self.attribute("color"));
        buf.push_optional_style("font-family", self.attribute("font-family"));
        buf.push_optional_style("font-size", self.attribute("font-size"));
        buf.push_optional_style("font-weight", self.attribute("font-weight"));
        buf.push_optional_style("line-height", self.attribute("line-height"));
        buf.push_optional_style("line-spacing", self.attribute("line-spacing"));
        buf.push_str("margin:0;");
        buf.push_optional_style("text-decoration", self.attribute("text-decoration"));
        buf.push_optional_style("text-transform", self.attribute("text-transform"));
        buf.push_optional_style("padding", self.attribute("inner-padding"));
        buf.push_str("mso-padding-alt:0px;");
        buf.push_optional_style("border-radius", self.attribute("border-radius"));
        buf.push('"');
        buf.push('>');
        // TODO
        if href.is_some() {
            buf.push_str("</a>");
        } else {
            buf.push_str("</p>");
        }
        buf.push_str("</td>");
        buf.push_str("</tr>");
        buf.push_str("</table");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJButton {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJButtonRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
