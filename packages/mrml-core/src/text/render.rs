use super::Text;
use crate::prelude::render::*;

impl<'element, 'header> Render<'element, 'header> for Renderer<'element, 'header, Text, ()> {
    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        cursor.buffer.push_str(self.element.inner_str());
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Text {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(Renderer::new(context, self, ()))
    }
}
