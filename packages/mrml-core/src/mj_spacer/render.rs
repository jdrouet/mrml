use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjSpacer, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::{Error, Header, Render, RenderBuffer, RenderOptions, Renderable, Tag};

struct MjSpacerRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjSpacer,
    container_width: Option<Pixel>,
}

impl<'e, 'h> Render<'e, 'h> for MjSpacerRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "height" => Some("20px"),
            _ => None,
        }
    }

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, _opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        Tag::div()
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("line-height", self.attribute("height"))
            .render_text(buf, "&#8202;");
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSpacer {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjSpacerRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-spacer");
}
