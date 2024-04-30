use super::Text;
use crate::prelude::render::*;

struct TextRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e Text,
}

impl<'e, 'h> Render<'e, 'h> for TextRender<'e, 'h> {
    fn header(&self) -> &'h Header<'h> {
        self.header
    }

    fn render(
        &self,
        _opts: &RenderOptions,
        _header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        buf.push_str(self.element.inner_str());
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for Text {
    fn is_raw(&'e self) -> bool {
        true
    }

    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(TextRender::<'e, 'h> {
            element: self,
            header,
        })
    }
}
