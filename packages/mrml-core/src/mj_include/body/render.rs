use super::{MjIncludeBody, MjIncludeBodyChild};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl MjIncludeBodyChild {
    pub(crate) fn as_renderable<'render, 'root: 'render>(
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

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let siblings = self.element.0.children.len();
        let raw_siblings = self
            .element
            .0
            .children
            .iter()
            .filter(|child| child.is_raw())
            .count();
        for (index, child) in self.element.0.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(raw_siblings);
            renderer.set_container_width(self.container_width);
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
    use crate::mj_include::body::{
        MjIncludeBody, MjIncludeBodyAttributes, MjIncludeBodyChild, MjIncludeBodyKind,
    };
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
            let header = Header::new(mj_head.as_ref(), "und", "auto");
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();
            let elt = MjText::default();
            let renderer = elt.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        let result: String = {
            let header = Header::new(mj_head.as_ref(), "und", "auto");
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();
            let elt = MjIncludeBody::new(
                MjIncludeBodyAttributes::new("memory:foo.mjml"),
                vec![MjIncludeBodyChild::MjText(MjText::default())],
            );
            let renderer = elt.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        assert_eq!(expected, result);
    }

    // Nests the include directly under `mj-body` rather than `mj-section`.
    // `mj-section` still treats a whole `MjIncludeBody` as a single non-raw
    // child (a separate, out-of-scope defect noted in the task), which would
    // make it wrap the fragment in one Outlook `<td>` instead of one per
    // column and mask the comparison below in an unrelated way. `mj-body`
    // forwards siblings/raw_siblings/container_width the same way without
    // that per-child wrapping, so it isolates the fix under test.
    #[cfg(feature = "parse")]
    fn render_via_include(fragment: &str) -> (String, String) {
        use crate::mjml::Mjml;
        use crate::prelude::parser::memory_loader::MemoryIncludeLoader;
        use crate::prelude::parser::ParserOptions;
        use crate::prelude::render::RenderOptions;

        let included = Mjml::parse_with_options(
            r#"<mjml><mj-body><mj-include path="inc.mjml" /></mj-body></mjml>"#,
            &ParserOptions {
                include_loader: Box::new(MemoryIncludeLoader::from(vec![("inc.mjml", fragment)])),
            },
        )
        .unwrap();
        let inlined = Mjml::parse(format!("<mjml><mj-body>{fragment}</mj-body></mjml>")).unwrap();

        (
            inlined.element.render(&RenderOptions::default()).unwrap(),
            included.element.render(&RenderOptions::default()).unwrap(),
        )
    }

    #[test]
    #[cfg(feature = "parse")]
    fn forwards_siblings_to_children() {
        // Mirrors the reported repro: a trailing comment must count as a
        // (non-raw-counted) sibling of the column, not be ignored, so the
        // column keeps the same width whether included or written inline.
        let fragment = "<mj-column><mj-text>hi</mj-text></mj-column>\n<!-- a comment -->";
        let (inlined, included) = render_via_include(fragment);
        html_compare::assert_similar(&inlined, &included);
    }

    #[test]
    #[cfg(feature = "parse")]
    fn forwards_container_width_to_children() {
        // container_width must reach the image the same way it would if the
        // fragment had been written inline, or the image loses its `width`.
        let fragment = r#"<mj-column><mj-image src="http://example.com/image.jpg" /></mj-column>"#;
        let (inlined, included) = render_via_include(fragment);
        html_compare::assert_similar(&inlined, &included);
    }

    #[test]
    fn basic_html_kind() {
        let opts = RenderOptions::default();
        let mj_head = Some(MjHead::default());

        let expected: String = {
            let header = Header::new(mj_head.as_ref(), "und", "auto");
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
            let header = Header::new(mj_head.as_ref(), "und", "auto");
            let context = RenderContext::new(&opts, header);
            let mut cursor = RenderCursor::default();

            let mut node = Node::from("span");
            node.children
                .push(MjBodyChild::Text(Text::from("Hello World!")));

            let elt = MjIncludeBody::new(
                MjIncludeBodyAttributes::new("memory:foo.html").with_kind(MjIncludeBodyKind::Html),
                vec![MjIncludeBodyChild::Node(node)],
            );

            let renderer = elt.renderer(&context);
            renderer.render(&mut cursor).unwrap();
            cursor.buffer.into()
        };
        assert_eq!(expected, result);
    }
}
