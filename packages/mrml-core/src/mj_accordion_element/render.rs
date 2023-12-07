use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjAccordionElement, NAME};
use crate::helper::condition::negation_conditional_tag;
use crate::helper::tag::Tag;
use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

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
    fn render_title(&self, opts: &RenderOptions) -> Result<String, Error> {
        if let Some(ref child) = self.element.children.title {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        } else {
            let child = MjAccordionTitle::default();
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        }
    }

    fn render_text(&self, opts: &RenderOptions) -> Result<String, Error> {
        if let Some(ref child) = self.element.children.text {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        } else {
            let child = MjAccordionText::default();
            let mut renderer = child.renderer(Rc::clone(&self.header));
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(opts)
        }
    }

    fn render_children(&self, opts: &RenderOptions) -> Result<String, Error> {
        Ok(self.render_title(opts)? + &self.render_text(opts)?)
    }
}

impl<'e, 'h> Render<'h> for MjAccordionElementRender<'e, 'h> {
    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.extra)
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

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let input = negation_conditional_tag(
            Tag::new("input")
                .add_attribute("type", "checkbox")
                .add_class("mj-accordion-checkbox")
                .add_style("display", "none")
                .closed(),
        );
        let div = Tag::div().render(self.render_children(opts)?);
        let label = Tag::new("label")
            .add_class("mj-accordion-element")
            .add_style("font-size", "13px")
            .maybe_add_style("font-family", self.attribute("font-family"))
            .render(input + &div);
        let td = Tag::td()
            .add_style("padding", "0px")
            .maybe_add_style("background-color", self.attribute("background-color"))
            .render(label);
        let tr = Tag::tr()
            .maybe_add_class(self.attribute("css-class"))
            .render(td);
        Ok(tr)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjAccordionElement {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
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
        let _rendered = renderer.render(&opts).unwrap();
    }
}
