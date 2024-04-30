use super::{MjRaw, MjRawChild, NAME};
use crate::helper::size::Pixel;
use crate::prelude::render::*;

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjRawChild {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        match self {
            Self::Comment(elt) => elt.renderer(header),
            Self::Node(elt) => elt.renderer(header),
            Self::Text(elt) => elt.renderer(header),
        }
    }
}

struct MjRawRender<'e, 'h> {
    header: &'h Header<'h>,
    element: &'e MjRaw,
    container_width: Option<Pixel>,
}

impl<'e, 'h> Render<'e, 'h> for MjRawRender<'e, 'h> {
    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> &'h Header<'h> {
        self.header
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(
        &self,
        opts: &RenderOptions,
        header: &mut VariableHeader,
        buf: &mut RenderBuffer,
    ) -> Result<(), Error> {
        let siblings = self.element.children.len();
        for (index, child) in self.element.children.iter().enumerate() {
            let mut renderer = child.renderer(self.header);
            renderer.set_index(index);
            renderer.set_siblings(siblings);
            renderer.set_raw_siblings(siblings);
            renderer.set_container_width(self.container_width.clone());
            renderer.render(opts, header, buf)?;
        }
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjRaw {
    fn renderer(&'e self, header: &'h Header<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        Box::new(MjRawRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(basic, "mj-raw");
    crate::should_render!(in_head, "mj-raw-head");
}
