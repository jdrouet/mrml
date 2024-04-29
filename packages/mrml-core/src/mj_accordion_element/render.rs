use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjAccordionElement, NAME};
use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderBuffer, RenderOptions, Renderable, Tag};

const CHILDREN_ATTRIBUTES: [&str; 9] = [
    "border",
    "icon-align",
    "icon-width",
    "icon-height",
    "icon-position",
    "icon-wrapped-url",
    "icon-wrapped-alt",
    "icon-unwrapped-url",
    "icon-unwrapped-alt",
];

struct MjAccordionElementRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjAccordionElement,
    extra: Map<String, String>,
}

impl<'e, 'h> MjAccordionElementRender<'e, 'h> {
    fn render_title(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        if let Some(ref child) = self.element.children.title {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts, buf)
        } else {
            let child = MjAccordionTitle::default();
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts, buf)
        }
    }

    fn render_text(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        if let Some(ref child) = self.element.children.text {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts, buf)
        } else {
            let child = MjAccordionText::default();
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts, buf)
        }
    }

    fn render_children(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        self.render_title(opts, buf)?;
        self.render_text(opts, buf)?;

        Ok(())
    }
}

impl<'e, 'h> Render<'e, 'h> for MjAccordionElementRender<'e, 'h> {
    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&str> {
        self.extra.get(key).map(|v| v.as_str())
    }

    fn raw_attribute(&self, key: &str) -> Option<&'e str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions, buf: &mut RenderBuffer) -> Result<(), Error> {
        let input = Tag::new("input")
            .add_attribute("type", "checkbox")
            .add_class("mj-accordion-checkbox")
            .add_style("display", "none");
        let div = Tag::div();
        let label = Tag::new("label")
            .add_class("mj-accordion-element")
            .add_style("font-size", "13px")
            .maybe_add_style("font-family", self.attribute("font-family"));
        let td = Tag::td()
            .add_style("padding", "0px")
            .maybe_add_style("background-color", self.attribute("background-color"));
        let tr = Tag::tr().maybe_add_class(self.attribute("css-class"));

        tr.render_open(buf);
        td.render_open(buf);
        label.render_open(buf);
        buf.start_negation_conditional_tag();
        input.render_closed(buf);
        buf.end_negation_conditional_tag();
        div.render_open(buf);
        self.render_children(opts, buf)?;
        div.render_close(buf);
        label.render_close(buf);
        td.render_close(buf);
        tr.render_close(buf);

        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionElement {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjAccordionElementRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::mj_accordion_element::{MjAccordionElement, MjAccordionElementChildren};
    use crate::mj_accordion_text::MjAccordionText;
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::prelude::render::*;
    use crate::text::Text;

    #[test]
    fn basic() {
        let opts = RenderOptions::default();
        let head = Rc::new(RefCell::new(Header::new(&None)));
        let element = MjAccordionElement {
            attributes: Default::default(),
            children: MjAccordionElementChildren {
                title: Some(MjAccordionTitle {
                    attributes: Default::default(),
                    children: vec![Text::from("Hello World!".to_string())],
                }),
                text: Some(MjAccordionText {
                    attributes: Default::default(),
                    children: vec![Text::from("Lorem Ipsum".to_string()).into()],
                }),
            },
        };
        let renderer = element.renderer(head);
        let mut buf = RenderBuffer::default();
        renderer.render(&opts, &mut buf).unwrap();
    }
}
