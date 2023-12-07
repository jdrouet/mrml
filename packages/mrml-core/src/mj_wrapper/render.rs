use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjWrapper, NAME};
use crate::helper::condition::{END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::mj_section::{SectionLikeRender, WithMjSectionBackground};
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

struct MjWrapperRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjWrapper,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MjWrapperRender<'e, 'h> {
    fn current_width(&self) -> Option<Pixel> {
        self.container_width.as_ref().map(|width| {
            let hborder = self.get_border_horizontal();
            let hpadding = self.get_padding_horizontal();
            Pixel::new(width.value() - hborder.value() - hpadding.value())
        })
    }
}

impl<'e, 'h> WithMjSectionBackground<'h> for MjWrapperRender<'e, 'h> {}

impl<'e, 'h> SectionLikeRender<'h> for MjWrapperRender<'e, 'h> {
    fn clone_header(&self) -> Rc<RefCell<Header<'h>>> {
        Rc::clone(&self.header)
    }

    fn children(&self) -> &Vec<crate::mj_body::MjBodyChild> {
        &self.element.children
    }

    fn container_width(&self) -> &Option<Pixel> {
        &self.container_width
    }

    fn render_wrapped_children(&self, opts: &RenderOptions) -> Result<String, Error> {
        let tr = Tag::tr();
        let siblings = self.get_siblings();
        let raw_siblings = self.get_raw_siblings();
        let current_width = self.current_width();
        let container_width = self.container_width.as_ref().map(|v| v.to_string());
        let mut result = String::default();
        for child in self.children().iter() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_container_width(current_width.clone());
            if child.is_raw() {
                result.push_str(&renderer.render(opts)?);
            } else {
                let td = renderer
                    .set_style("td-outlook", Tag::td())
                    .maybe_add_attribute("align", renderer.attribute("align"))
                    .maybe_add_attribute("width", container_width.as_ref().cloned())
                    .maybe_add_suffixed_class(renderer.attribute("css-class"), "outlook");
                result.push_str(&tr.open());
                result.push_str(&td.open());
                result.push_str(END_CONDITIONAL_TAG);
                result.push_str(&renderer.render(opts)?);
                result.push_str(START_CONDITIONAL_TAG);
                result.push_str(&td.close());
                result.push_str(&tr.close());
            }
        }
        Ok(result)
    }
}

impl<'e, 'h> Render<'h> for MjWrapperRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "background-position" => Some("top center"),
            "background-repeat" => Some("repeat"),
            "background-size" => Some("auto"),
            "direction" => Some("ltr"),
            "padding" => Some("20px 0"),
            "text-align" => Some("center"),
            "text-padding" => Some("4px 4px 4px 0"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        if self.is_full_width() {
            self.render_full_width(opts)
        } else {
            self.render_simple(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjWrapper {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjWrapperRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-wrapper");
    crate::should_render!(background, "mj-wrapper-background");
    crate::should_render!(border, "mj-wrapper-border");
    crate::should_render!(other, "mj-wrapper-other");
    crate::should_render!(padding, "mj-wrapper-padding");
}
