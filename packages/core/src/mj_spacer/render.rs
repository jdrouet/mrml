use super::{MJSpacer, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct MJSpacerRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJSpacer,
    container_width: Option<Pixel>,
}

impl<'e, 'h> MJSpacerRender<'e, 'h> {
    fn set_style_div(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("height", self.attribute("height"))
    }
}

impl<'e, 'h> Render<'h> for MJSpacerRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
        match key {
            "height" => Some("20px"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&HashMap<String, String>> {
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

    fn render(&self) -> Result<String, Error> {
        let height = self.attribute_as_pixel("height");
        let table = Tag::table_presentation();
        let tr = Tag::tr();
        let td = Tag::td()
            .maybe_add_style("height", height.as_ref().map(|v| v.to_string()))
            .maybe_add_attribute("height", height.as_ref().map(|h| h.value().to_string()));
        let div = self.set_style_div(Tag::div());
        let before = conditional_tag(table.open() + &tr.open() + &td.open());
        let after = conditional_tag(td.close() + &tr.close() + &table.close());
        Ok(before + &div.render("&nbsp;") + &after)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJSpacer {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJSpacerRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::compare;
    use crate::mjml::MJML;

    #[test]
    fn basic() {
        let template = include_str!("../../resources/compare/success/mj-spacer.mjml");
        let expected = include_str!("../../resources/compare/success/mj-spacer.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }
}
