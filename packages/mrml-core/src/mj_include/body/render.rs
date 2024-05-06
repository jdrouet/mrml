use super::{MjIncludeBody, MjIncludeBodyChild};
use crate::prelude::render::*;

impl MjIncludeBodyChild {
    pub fn as_renderable<'render, 'root: 'render>(
        &'root self,
    ) -> &'root (dyn Renderable<'render, 'root> + 'root) {
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

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjIncludeBodyChild {
    fn is_raw(&self) -> bool {
        self.as_renderable().is_raw()
    }

    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        self.as_renderable().renderer(context)
    }
}

impl<'root> Render<'root> for Renderer<'root, MjIncludeBody, ()> {
    fn raw_attribute(&self, _: &str) -> Option<&'root str> {
        None
    }

    fn default_attribute(&self, _: &str) -> Option<&'static str> {
        None
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(self.element.children.len());
            renderer.render(cursor)?;
        }
        Ok(())
    }
}

impl<'render, 'root: 'render> Renderable<'render, 'root> for MjIncludeBody {
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_body::MjBodyChild;
    use crate::mj_head::MjHead;
    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyChild, MjIncludeBodyKind};
    use crate::mj_raw::{MjRaw, MjRawChild};
    use crate::mj_text::MjText;
    use crate::node::Node;
    use crate::prelude::render::{Header, RenderContext, RenderCursor, RenderOptions, Renderable};
    use crate::text::Text;

    #[test]
    fn basic_mjml_kind() {
        let opts = RenderOptions::default();
        let mj_head = Some(MjHead::default());
        let expected: String = {
            let header = Header::new(mj_head.as_ref(), None);
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();
            let elt = MjText::default();
            let renderer = elt.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        let result: String = {
            let header = Header::new(mj_head.as_ref(), None);
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();
            let mut elt = MjIncludeBody::default();
            elt.attributes.path = "memory:foo.mjml".to_string();
            elt.children
                .push(MjIncludeBodyChild::MjText(MjText::default()));
            let renderer = elt.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        assert_eq!(expected, result);
    }

    #[test]
    fn basic_html_kind() {
        let opts = RenderOptions::default();
        let mj_head = Some(MjHead::default());

        let expected: String = {
            let header = Header::new(mj_head.as_ref(), None);
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();

            let mut node = Node::from("span");
            node.children
                .push(MjRawChild::Text(Text::from("Hello World!")));

            let mut root = MjRaw::default();
            root.children.push(MjRawChild::Node(node));
            let renderer = root.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        let result: String = {
            let header = Header::new(mj_head.as_ref(), None);
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();

            let mut node = Node::from("span");
            node.children
                .push(MjBodyChild::Text(Text::from("Hello World!")));

            let mut elt = MjIncludeBody::default();
            elt.attributes.kind = MjIncludeBodyKind::Html;
            elt.attributes.path = "memory:foo.html".to_string();
            elt.children.push(MjIncludeBodyChild::Node(node));

            let renderer = elt.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        assert_eq!(expected, result);
    }
}
