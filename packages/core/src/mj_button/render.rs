use super::{MJButton, NAME};
use crate::helper::size::Pixel;
use crate::helper::spacing::Spacing;
use crate::prelude::render::{Error, Header, Render, Renderable};
use crate::{write_attribute, write_optional_attribute, write_optional_style, write_style};
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

    fn render(&self, buf: &mut String) -> Result<(), Error> {
        if let Some(ref font_family) = self.attribute("font-family") {
            self.header.borrow_mut().add_used_font_family(font_family);
        }
        buf.push_str("<table role=\"presentation\" style=\"border-collapse:separate;");
        if let Some(width) = self.attribute("width") {
            buf.push_str("width:");
            buf.push_str(width.as_str());
            buf.push(';');
        }
        buf.push_str("line-height:100%\">");
        buf.push_str("<tr>");
        buf.push_str("<td align=\"center\"");
        write_optional_attribute!(buf, "bgcolor", self.attribute("background-color"));
        buf.push_str(" role=\"presentation\"");
        write_optional_attribute!(buf, "valign", self.attribute("vertical-align"));
        buf.push_str(">");
        let href = self.attribute("href");
        if let Some(ref href) = href {
            buf.push_str("<a href=\"");
            buf.push_str(href);
            buf.push('"');
            write_optional_attribute!(buf, "target", self.attribute("target"));
        } else {
            buf.push_str("<p");
        }
        write_optional_attribute!(buf, "rel", self.attribute("rel"));
        write_optional_attribute!(buf, "name", self.attribute("name"));
        buf.push_str(" style=\"display:inline-block;");
        write_optional_style!(buf, "width", self.content_width());
        write_optional_style!(buf, "background", self.attribute("background-color"));
        write_optional_style!(buf, "color", self.attribute("color"));
        write_optional_style!(buf, "font-family", self.attribute("font-family"));
        write_optional_style!(buf, "font-size", self.attribute("font-size"));
        write_optional_style!(buf, "font-weight", self.attribute("font-weight"));
        write_optional_style!(buf, "line-height", self.attribute("line-height"));
        write_optional_style!(buf, "line-spacing", self.attribute("line-spacing"));
        buf.push_str("margin:0;");
        write_optional_style!(buf, "text-decoration", self.attribute("text-decoration"));
        write_optional_style!(buf, "text-transform", self.attribute("text-transform"));
        write_optional_style!(buf, "padding", self.attribute("inner-padding"));
        buf.push_str("mso-padding-alt:0px;");
        write_optional_style!(buf, "border-radius", self.attribute("border-radius"));
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
