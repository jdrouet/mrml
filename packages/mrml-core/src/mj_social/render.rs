use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjSocial, MjSocialChild, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::size::{Pixel, Size};
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSocialChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MjSocialElement(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}

const EXTRA_CONTAINER_KEY: [&str; 13] = [
    "inner-padding",
    "border-radius",
    "color",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "icon-size",
    "icon-height",
    "icon-padding",
    "text-padding",
    "line-height",
    "text-decoration",
];
const EXTRA_CHILD_KEY: [&str; 13] = [
    "padding",
    "border-radius",
    "color",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "icon-size",
    "icon-height",
    "icon-padding",
    "text-padding",
    "line-height",
    "text-decoration",
];

struct MjSocialRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjSocial,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

impl<'e, 'h> MjSocialRender<'e, 'h> {
    fn set_style_table_vertical(&self, tag: Tag) -> Tag {
        tag.add_style("margin", "0px")
    }

    fn is_horizontal(&self) -> bool {
        self.attribute("mode")
            .map(|mode| mode == "horizontal")
            .unwrap_or(true)
    }

    fn build_child_attributes(&self) -> Vec<(&str, String)> {
        EXTRA_CONTAINER_KEY
            .iter()
            .zip(EXTRA_CHILD_KEY.iter())
            .filter_map(|(con_key, child_key)| {
                self.attribute(con_key).map(|value| (*child_key, value))
            })
            .collect::<Vec<_>>()
    }

    fn render_horizontal(&self, opts: &RenderOptions) -> Result<String, Error> {
        let table = Tag::table_presentation().maybe_add_attribute("align", self.attribute("align"));
        let tr = Tag::tr();
        let td = Tag::td();
        let inner_table = Tag::table_presentation()
            .maybe_add_attribute("align", self.attribute("align"))
            .add_style("float", "none")
            .add_style("display", "inline-table");
        let inner_tbody = Tag::tbody();
        let before = conditional_tag(table.open() + &tr.open());
        let after = conditional_tag(tr.close() + &table.close());
        let child_attributes = self.build_child_attributes();
        let content = self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                child_attributes.iter().for_each(|(key, value)| {
                    renderer.add_extra_attribute(key, value);
                });
                Ok(res
                    + &conditional_tag(td.open())
                    + &inner_table.render(inner_tbody.render(renderer.render(opts)?))
                    + &conditional_tag(td.close()))
            },
        )?;
        Ok(before + &content + &after)
    }

    fn render_vertical(&self, opts: &RenderOptions) -> Result<String, Error> {
        let table = self.set_style_table_vertical(Tag::table_presentation());
        let child_attributes = self.build_child_attributes();
        let content = self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                child_attributes.iter().for_each(|(key, value)| {
                    renderer.add_extra_attribute(key, value);
                });
                Ok(res + &renderer.render(opts)?)
            },
        )?;
        Ok(table.render(Tag::tbody().render(content)))
    }
}

impl<'e, 'h> Render<'h> for MjSocialRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "align" => Some("center"),
            "border-radius" => Some("3px"),
            "color" => Some("#333333"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "icon-size" => Some("20px"),
            "line-height" => Some("22px"),
            "mode" => Some("horizontal"),
            "padding" => Some("10px 25px"),
            "text-decoration" => Some("none"),
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

    fn get_width(&self) -> Option<Size> {
        self.container_width
            .as_ref()
            .map(|w| Size::Pixel(w.clone()))
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let font_families = self.attribute("font-family").unwrap_or_default(); // never happens
        self.header.borrow_mut().add_font_families(font_families);
        if self.is_horizontal() {
            self.render_horizontal(opts)
        } else {
            self.render_vertical(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSocial {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjSocialRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-social");
    crate::should_render!(align, "mj-social-align");
    crate::should_render!(border_radius, "mj-social-border-radius");
    crate::should_render!(class, "mj-social-class");
    crate::should_render!(color, "mj-social-color");
    crate::should_render!(
        container_background_color,
        "mj-social-container-background-color"
    );
    crate::should_render!(font_family, "mj-social-font-family");
    crate::should_render!(font, "mj-social-font");
    crate::should_render!(icon, "mj-social-icon");
    crate::should_render!(link, "mj-social-link");
    crate::should_render!(mode, "mj-social-mode");
    crate::should_render!(padding, "mj-social-padding");
    crate::should_render!(text, "mj-social-text");
}
