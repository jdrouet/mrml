use super::{MjAccordionElement, NAME};
use crate::mj_accordion_text::MjAccordionText;
use crate::mj_accordion_title::MjAccordionTitle;
use crate::prelude::hash::Map;
use crate::prelude::render::*;

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

struct MjAccordionElementExtra<'a> {
    attributes: Map<&'a str, &'a str>,
}

impl<'root> Renderer<'root, MjAccordionElement, MjAccordionElementExtra<'root>> {
    fn render_title(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if let Some(ref child) = self.element.children.title {
            let mut renderer = child.renderer(self.context());
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(cursor)
        } else {
            let child = MjAccordionTitle::default();
            let mut renderer = child.renderer(self.context());
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(cursor)
        }
    }

    fn render_text(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        if let Some(ref child) = self.element.children.text {
            let mut renderer = child.renderer(self.context());
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(cursor)
        } else {
            let child = MjAccordionText::default();
            let mut renderer = child.renderer(self.context());
            CHILDREN_ATTRIBUTES.iter().for_each(|name| {
                renderer.maybe_add_extra_attribute(name, self.attribute(name));
            });
            renderer.render(cursor)
        }
    }

    fn render_children(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        self.render_title(cursor)?;
        self.render_text(cursor)?;

        Ok(())
    }
}

impl<'root> Render<'root> for Renderer<'root, MjAccordionElement, MjAccordionElementExtra<'root>> {
    fn add_extra_attribute(&mut self, key: &'root str, value: &'root str) {
        self.extra.attributes.insert(key, value);
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&'root str> {
        self.extra.attributes.get(key).copied()
    }

    fn raw_attribute(&self, key: &str) -> Option<&'root str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
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

        tr.render_open(&mut cursor.buffer)?;
        td.render_open(&mut cursor.buffer)?;
        label.render_open(&mut cursor.buffer)?;
        cursor.buffer.start_negation_conditional_tag();
        input.render_closed(&mut cursor.buffer)?;
        cursor.buffer.end_negation_conditional_tag();
        div.render_open(&mut cursor.buffer)?;
        self.render_children(cursor)?;
        div.render_close(&mut cursor.buffer);
        label.render_close(&mut cursor.buffer);
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);

        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjAccordionElement {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(
            context,
            self,
            MjAccordionElementExtra {
                attributes: Map::new(),
            },
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_accordion_element::{MjAccordionElement, MjAccordionElementChildren};
    use crate::mj_accordion_text::MjAccordionText;
    use crate::mj_accordion_title::MjAccordionTitle;
    use crate::prelude::render::*;
    use crate::text::Text;

    #[test]
    fn basic() {
        let opts = RenderOptions::default();
        let head = Header::new(None, None);
        let ctx = RenderContext::new(&opts, head);

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
        let renderer = element.renderer(&ctx);
        let mut cursor = RenderCursor::default();
        renderer.render(&mut cursor).unwrap();
    }
}
