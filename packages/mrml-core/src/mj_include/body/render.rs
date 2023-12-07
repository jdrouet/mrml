use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::{MjIncludeBody, MjIncludeBodyChild};
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

impl MjIncludeBodyChild {
    pub fn as_renderable<'r, 'e: 'r, 'h: 'r>(&'e self) -> &'e (dyn Renderable<'r, 'e, 'h> + 'e) {
        match self {
            Self::Comment(elt) => elt,
            Self::MjAccordion(elt) => elt,
            Self::MjButton(elt) => elt,
            Self::MjCarousel(elt) => elt,
            Self::MjColumn(elt) => elt,
            Self::MjDivider(elt) => elt,
            Self::MjGroup(elt) => elt,
            Self::MjHero(elt) => elt,
            Self::MjImage(elt) => elt,
            Self::MjNavbar(elt) => elt,
            Self::MjRaw(elt) => elt,
            Self::MjSection(elt) => elt,
            Self::MjSocial(elt) => elt,
            Self::MjSpacer(elt) => elt,
            Self::MjTable(elt) => elt,
            Self::MjText(elt) => elt,
            Self::MjWrapper(elt) => elt,
            Self::Node(elt) => elt,
            Self::Text(elt) => elt,
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjIncludeBodyChild {
    fn is_raw(&self) -> bool {
        self.as_renderable().is_raw()
    }

    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        self.as_renderable().renderer(header)
    }
}

struct MjIncludeBodyRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjIncludeBody,
}

impl<'e, 'h> Render<'h> for MjIncludeBodyRender<'e, 'h> {
    fn attributes(&self) -> Option<&Map<String, String>> {
        None
    }

    fn default_attribute(&self, _key: &str) -> Option<&str> {
        None
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let mut children = String::default();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(Rc::clone(&self.header));
            renderer.set_index(index);
            renderer.set_siblings(self.element.children.len());
            children.push_str(&renderer.render(opts)?);
        }
        Ok(children)
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjIncludeBody {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjIncludeBodyRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::mj_body::MjBodyChild;
    use crate::mj_head::MjHead;
    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyChild, MjIncludeBodyKind};
    use crate::mj_raw::{MjRaw, MjRawChild};
    use crate::mj_text::MjText;
    use crate::node::Node;
    use crate::prelude::render::{Header, RenderOptions, Renderable};
    use crate::text::Text;

    #[test]
    fn basic_mjml_kind() {
        let opts = RenderOptions::default();
        let mj_head = Some(MjHead::default());
        let expected = {
            let header = Rc::new(RefCell::new(Header::new(&mj_head)));
            let elt = MjText::default();
            let renderer = elt.renderer(header);
            renderer.render(&opts).unwrap()
        };
        let result = {
            let header = Rc::new(RefCell::new(Header::new(&mj_head)));
            let mut elt = MjIncludeBody::default();
            elt.attributes.path = "memory:foo.mjml".to_string();
            elt.children
                .push(MjIncludeBodyChild::MjText(MjText::default()));
            let renderer = elt.renderer(header);
            renderer.render(&opts).unwrap()
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn basic_html_kind() {
        let opts = RenderOptions::default();
        let mj_head = Some(MjHead::default());

        let expected = {
            let header = Rc::new(RefCell::new(Header::new(&mj_head)));

            let mut node = Node::new("span".to_string());
            node.children
                .push(MjRawChild::Text(Text::from("Hello World!")));

            let mut root = MjRaw::default();
            root.children.push(MjRawChild::Node(node));
            let renderer = root.renderer(header);
            renderer.render(&opts).unwrap()
        };
        let result = {
            let header = Rc::new(RefCell::new(Header::new(&mj_head)));

            let mut node = Node::new("span".to_string());
            node.children
                .push(MjBodyChild::Text(Text::from("Hello World!")));

            let mut elt = MjIncludeBody::default();
            elt.attributes.kind = MjIncludeBodyKind::Html;
            elt.attributes.path = "memory:foo.html".to_string();
            elt.children.push(MjIncludeBodyChild::Node(node));

            let renderer = elt.renderer(header);
            renderer.render(&opts).unwrap()
        };
        assert_eq!(expected, result);
    }
}
