use super::Fragment;
use crate::prelude::hash::Map;
use crate::prelude::render::*;

impl<'render, 'root: 'render, T> Render<'root>
    for Renderer<'root, Fragment<T>, Map<&'root str, &'root str>>
where
    T: Renderable<'render, 'root>,
{
    fn tag(&self) -> Option<&str> {
        None
    }

    fn context(&self) -> &'root RenderContext<'root> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        for child in self.element.children.iter() {
            let mut renderer = child.renderer(self.context);
            renderer.set_container_width(self.container_width.clone());
            self.extra.iter().for_each(|(key, value)| {
                renderer.add_extra_attribute(key, value);
            });
            renderer.render(cursor)?;
        }
        Ok(())
    }

    fn set_container_width(&mut self, width: Option<crate::helper::size::Pixel>) {
        self.container_width = width;
    }

    fn set_index(&mut self, index: usize) {
        self.index = index;
    }

    fn set_siblings(&mut self, count: usize) {
        self.siblings = count;
    }

    fn set_raw_siblings(&mut self, count: usize) {
        self.raw_siblings = count
    }

    fn add_extra_attribute(&mut self, key: &'root str, value: &'root str) {
        self.extra.insert(key, value);
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&'root str> {
        self.extra.get(key).copied()
    }
}

impl<'render, 'root: 'render, T: Renderable<'render, 'root>> Renderable<'render, 'root>
    for Fragment<T>
{
    fn renderer(
        &'root self,
        context: &'root RenderContext<'root>,
    ) -> Box<dyn Render<'root> + 'render> {
        Box::new(Renderer::new(context, self, Map::new()))
    }
}

#[cfg(test)]
mod tests {
    use crate::fragment::Fragment;
    use crate::text::Text;

    #[test]
    fn empty_script_should_have_closing_element() {
        use crate::mjml::Mjml;
        use crate::prelude::render::RenderOptions;

        let opts = RenderOptions::default();
        let mut root = Mjml::default();
        let body = crate::mj_body::MjBody {
            children: vec![Fragment::from(vec![
                Text::from("Hello World!").into(),
                Text::from("Second child!").into(),
            ])
            .into()],
            ..Default::default()
        };
        root.children.body = Some(body);
        let result = root.render(&opts).unwrap();
        assert!(result.contains("Hello World!"));
        assert!(result.contains("Second child!"));
    }
}
