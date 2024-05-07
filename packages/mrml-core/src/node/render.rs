use super::Node;
use crate::prelude::is_void_element;
use crate::prelude::render::*;

impl<'render, 'root: 'render, T> Render<'root> for Renderer<'root, Node<T>, ()>
where
    T: Renderable<'render, 'root>,
{
    fn tag(&self) -> Option<&str> {
        Some(self.element.tag.as_str())
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        cursor.buffer.open_tag(&self.element.tag);
        for (key, value) in self.element.attributes.iter() {
            cursor.buffer.push_attribute(key, value)?;
        }
        if self.element.children.is_empty() {
            if is_void_element(self.element.tag.as_str()) {
                cursor.buffer.closed_tag();
            } else {
                cursor.buffer.close_tag();
                cursor.buffer.end_tag(&self.element.tag);
            }
        } else {
            cursor.buffer.close_tag();
            for (index, child) in self.element.children.iter().enumerate() {
                // TODO children
                let mut renderer = child.renderer(self.context);
                renderer.set_index(index);
                renderer.render(cursor)?;
            }
            cursor.buffer.end_tag(&self.element.tag);
        }
        Ok(())
    }
}

impl<'render, 'root: 'render, T: Renderable<'render, 'root>> Renderable<'render, 'root>
    for Node<T>
{
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, ()))
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "parse")]
    #[test]
    fn empty_script_should_have_closing_element() {
        use crate::mjml::Mjml;
        use crate::prelude::render::RenderOptions;

        let opts = RenderOptions::default();
        let template = r#"<mjml>
    <mj-body>
        <mj-section>
        <mj-column>
            <mj-raw><script src="http://example.com/hello.js"></script></mj-raw>
            <mj-text>
            Hello World!
            </mj-text>
        </mj-column>
        </mj-section>
    </mj-body>
</mjml>"#;
        let root = Mjml::parse(template).unwrap();
        let result = root.render(&opts).unwrap();
        assert!(result.contains("<script src=\"http://example.com/hello.js\"></script>"));
    }
}
