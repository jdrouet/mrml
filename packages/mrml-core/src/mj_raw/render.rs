use super::{MjRaw, MjRawChild, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjRawChild {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(context),
            Self::Node(elt) => elt.renderer(context),
            Self::Text(elt) => elt.renderer(context),
        }
    }
}

struct MjRawRender<'e, 'h> {
    context: &'h RenderContext<'h>,
    element: &'e MjRaw,
    container_width: Option<Pixel>,
}

impl<'e, 'h> Render<'e, 'h> for MjRawRender<'e, 'h> {
    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn context(&self) -> &'h RenderContext<'h> {
        self.context
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, header: &mut VariableHeader, buf: &mut RenderBuffer) -> Result<(), Error> {
        let siblings = self.element.children.len();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.context());
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(siblings);
            renderer.set_container_width(self.container_width.clone());
            renderer.render(header, buf)?;
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjRaw {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjRawRender::<'e, 'h> {
            element: self,
            context,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-raw");
    crate::should_render!(in_head, "mj-raw-head");
}
