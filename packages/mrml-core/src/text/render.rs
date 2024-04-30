use super::Text;
use crate::prelude::render::*;

struct TextRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e Text,
}

impl<'e, 'h> Render<'e, 'h> for TextRender<'e, 'h> {
    fn context(&self) -> &'h RenderContext<'h> {
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
        Box::new(TextRender::<'e, 'h> {
            element: self,
            context,
        })
    }
}
