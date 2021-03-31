use super::{MJRaw, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::{Error, Header, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct MJRawRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJRaw,
    container_width: Option<Pixel>,
}

impl<'e, 'h> Render<'h> for MJRawRender<'e, 'h> {
    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self) -> Result<String, Error> {
        let siblings = self.element.children.len();
        self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                renderer.set_siblings(siblings);
                renderer.set_raw_siblings(siblings);
                renderer.set_container_width(self.container_width.clone());
                Ok(res + &renderer.render()?)
            },
        )
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJRaw {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJRawRender::<'e, 'h> {
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
        let template = include_str!("../../resources/compare/success/mj-raw.mjml");
        let expected = include_str!("../../resources/compare/success/mj-raw.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render().unwrap();
        compare(expected, result.as_str());
    }
}
