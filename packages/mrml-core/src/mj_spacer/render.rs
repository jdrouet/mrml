use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjSpacer, NAME};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjSpacerRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjSpacer,
    container_width: Option<Pixel>,
}

impl<'e, 'h> Render<'h> for MjSpacerRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "height" => Some("20px"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
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

    fn render(&self, _opts: &RenderOptions) -> Result<String, Error> {
        Ok(Tag::div()
            .maybe_add_style("height", self.attribute("height"))
            .maybe_add_style("line-height", self.attribute("height"))
            .render("&#8202;"))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSpacer {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
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
