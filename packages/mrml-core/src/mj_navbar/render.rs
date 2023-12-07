use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjNavbar, MjNavbarChild, NAME};
use crate::helper::condition::{conditional_tag, mso_negation_conditional_tag};
use crate::helper::size::{Pixel, Size};
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjNavbarChild {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        match self {
            Self::MjNavbarLink(elt) => elt.renderer(header),
            Self::Comment(elt) => elt.renderer(header),
        }
    }
}

struct MjNavbarRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjNavbar,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
    id: String,
}

impl<'e, 'h> MjNavbarRender<'e, 'h> {
    fn set_style_input(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none !important")
            .add_style("max-height", "0")
            .add_style("visibility", "hidden")
    }

    fn set_style_label(&self, tag: Tag) -> Tag {
        tag.add_style("display", "block")
            .add_style("cursor", "pointer")
            .add_style("mso-hide", "all")
            .add_style("-moz-user-select", "none")
            .add_style("user-select", "none")
            .maybe_add_style("color", self.attribute("ico-color"))
            .maybe_add_style("font-size", self.attribute("ico-font-size"))
            .maybe_add_style("font-family", self.attribute("ico-font-family"))
            .maybe_add_style("text-transform", self.attribute("ico-text-transform"))
            .maybe_add_style("text-decoration", self.attribute("ico-text-decoration"))
            .maybe_add_style("line-height", self.attribute("ico-line-height"))
            .maybe_add_style("padding-top", self.attribute("ico-padding-top"))
            .maybe_add_style("padding-right", self.attribute("ico-padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("ico-padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("ico-padding-left"))
            .maybe_add_style("padding", self.attribute("ico-padding"))
    }

    fn set_style_trigger(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none")
            .add_style("max-height", "0px")
            .add_style("max-width", "0px")
            .add_style("font-size", "0px")
            .add_style("overflow", "hidden")
    }

    fn set_style_ico_close(&self, tag: Tag) -> Tag {
        tag.add_style("display", "none")
            .add_style("mso-hide", "all")
    }

    fn set_style_ico_open(&self, tag: Tag) -> Tag {
        tag.add_style("mso-hide", "all")
    }

    fn has_hamburger(&self) -> bool {
        self.attribute("hamburger")
            .and_then(|value| {
                if value == "hamburger" {
                    Some(true)
                } else {
                    None
                }
            })
            .is_some()
    }

    fn render_hamburger(&self) -> String {
        let input = self
            .set_style_input(Tag::new("input"))
            .add_class("mj-menu-checkbox")
            .add_attribute("id", self.id.clone())
            .add_attribute("type", "checkbox");
        let div = self
            .set_style_trigger(Tag::div())
            .add_class("mj-menu-trigger");
        let label = self
            .set_style_label(Tag::new("label"))
            .maybe_add_attribute("align", self.attribute("ico-align"))
            .add_class("mj-menu-label")
            .add_attribute("for", self.id.clone());
        let span_open = self
            .set_style_ico_open(Tag::new("span"))
            .add_class("mj-menu-icon-open");
        let span_close = self
            .set_style_ico_close(Tag::new("span"))
            .add_class("mj-menu-icon-close");
        let content = span_open.render(self.attribute("ico-open").unwrap_or_default())
            + &span_close.render(self.attribute("ico-close").unwrap_or_default());
        let content = div.render(label.render(content));
        mso_negation_conditional_tag(input.closed()) + &content
    }

    fn update_header(&self) {
        let style = format!(
            r#"
        noinput.mj-menu-checkbox {{ display:block!important; max-height:none!important; visibility:visible!important; }}
        @media only screen and (max-width:{}) {{
          .mj-menu-checkbox[type="checkbox"] ~ .mj-inline-links {{ display:none!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-inline-links,
          .mj-menu-checkbox[type="checkbox"] ~ .mj-menu-trigger {{ display:block!important; max-width:none!important; max-height:none!important; font-size:inherit!important; }}
          .mj-menu-checkbox[type="checkbox"] ~ .mj-inline-links > a {{ display:block!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-menu-trigger .mj-menu-icon-close {{ display:block!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-menu-trigger .mj-menu-icon-open {{ display:none!important; }}
        }}
        "#,
            self.header.borrow().breakpoint().lower().to_string()
        );
        self.header.borrow_mut().add_style(style);
    }
}

impl<'e, 'h> Render<'h> for MjNavbarRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "align" => Some("center"),
            "ico-align" => Some("center"),
            "ico-open" => Some("&#9776;"),
            "ico-close" => Some("&#8855;"),
            "ico-color" => Some("#000000"),
            "ico-font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "ico-font-size" => Some("30px"),
            "ico-text-transform" => Some("uppercase"),
            "ico-padding" => Some("10px"),
            "ico-text-decoration" => Some("none"),
            "ico-line-height" => Some("30px"),
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
        self.update_header();
        let div = Tag::div().add_class("mj-inline-links");
        let table = Tag::table_presentation().maybe_add_attribute("align", self.attribute("align"));
        let tr = Tag::tr();
        let base_url = self.attribute("base-url");
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.maybe_add_extra_attribute("navbar-base-url", base_url.clone());
                Ok(res + &renderer.render(opts)?)
            })?;
        let before = conditional_tag(table.open() + &tr.open());
        let after = conditional_tag(tr.close() + &table.close());
        let content = div.render(before + &content + &after);
        if self.has_hamburger() {
            Ok(self.render_hamburger() + &content)
        } else {
            Ok(content)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjNavbar {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        let id = header.borrow().next_id();
        Box::new(MjNavbarRender::<'e, 'h> {
            element: self,
            header,
            id,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-navbar");
    crate::should_render!(align_class, "mj-navbar-align-class");
    crate::should_render!(ico, "mj-navbar-ico");
}
