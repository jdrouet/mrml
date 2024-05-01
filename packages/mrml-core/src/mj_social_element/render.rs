use super::network::SocialNetwork;
use super::{MjSocialElement, NAME};
use crate::helper::size::{Pixel, Size};
use crate::prelude::hash::Map;
use crate::prelude::render::*;

const DEFAULT_ICON_ORIGIN: &str = "https://www.mailjet.com/images/theme/v1/icons/ico-social/";

#[derive(Default)]
struct MjSocialElementExtra {
    attributes: Map<String, String>,
    container_width: Option<Pixel>,
    network: Option<SocialNetwork>,
}

impl MjSocialElementExtra {
    pub fn new(network: Option<SocialNetwork>) -> Self {
        Self {
            attributes: Map::new(),
            container_width: None,
            network,
        }
    }
}

impl<'element, 'header> Renderer<'element, 'header, MjSocialElement, MjSocialElementExtra> {
    fn get_background_color(&self) -> Option<String> {
        self.attribute("background-color").or_else(|| {
            self.extra
                .network
                .as_ref()
                .map(|net| net.background_color().to_string())
        })
    }

    fn get_icon_size(&self) -> Option<Size> {
        self.attribute_as_size("icon-size")
    }

    fn get_icon_height(&self) -> Option<Size> {
        self.attribute_as_size("icon-height")
    }

    fn get_icon_src(&self) -> Option<String> {
        self.attribute("src").or_else(|| {
            self.extra.network.as_ref().map(|net| {
                if let Some(ref origin) = self.context.options.social_icon_origin {
                    net.icon_src(origin)
                } else {
                    net.icon_src(DEFAULT_ICON_ORIGIN)
                }
            })
        })
    }

    fn set_style_img<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("display", "block")
    }

    fn set_style_icon<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("padding", self.attribute("icon-padding"))
            .add_style("font-size", "0")
            .maybe_add_style(
                "height",
                self.get_icon_height()
                    .or_else(|| self.get_icon_size())
                    .map(|item| item.to_string()),
            )
            .add_style("vertical-align", "middle")
            .maybe_add_style("width", self.get_icon_size().map(|item| item.to_string()))
    }

    fn set_style_table<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("background", self.get_background_color())
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .maybe_add_style("width", self.get_icon_size().map(|size| size.to_string()))
    }

    fn set_style_td<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
    }

    fn set_style_td_text<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.add_style("vertical-align", "middle")
            .maybe_add_style("padding", self.attribute("text-padding"))
    }

    fn set_style_text<'a>(&self, tag: Tag<'a>) -> Tag<'a> {
        tag.maybe_add_style("color", self.attribute("color"))
            .maybe_add_style("font-size", self.attribute("font-size"))
            .maybe_add_style("font-weight", self.attribute("font-weight"))
            .maybe_add_style("font-style", self.attribute("font-style"))
            .maybe_add_style("font-family", self.attribute("font-family"))
            .maybe_add_style("line-height", self.attribute("line-height"))
            .maybe_add_style("text-decoration", self.attribute("text-decoration"))
    }

    fn get_href(&self) -> Option<String> {
        self.attribute("href")
            .map(|href| {
                self.extra
                    .network
                    .as_ref()
                    .and_then(|net| net.share_url(&href))
                    .or(Some(href))
            })
            .unwrap_or_default()
    }

    fn render_icon(&self, href: &Option<String>, cursor: &mut RenderCursor) {
        let table = self.set_style_table(Tag::table_presentation());
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_icon(Tag::td());
        let a = Tag::new("a")
            .maybe_add_attribute("href", href.clone())
            .maybe_add_attribute("rel", self.attribute("rel"))
            .maybe_add_attribute("target", self.attribute("target"));
        let img = self
            .set_style_img(Tag::new("img"))
            .maybe_add_attribute("alt", self.attribute("alt"))
            .maybe_add_attribute("title", self.attribute("title"))
            .maybe_add_attribute(
                "height",
                self.get_icon_height()
                    .or_else(|| self.get_icon_size())
                    .map(|size| size.value().to_string()),
            )
            .maybe_add_attribute("src", self.get_icon_src())
            .maybe_add_attribute(
                "width",
                self.get_icon_size().map(|size| size.value().to_string()),
            );

        table.render_open(&mut cursor.buffer);
        tbody.render_open(&mut cursor.buffer);
        tr.render_open(&mut cursor.buffer);
        td.render_open(&mut cursor.buffer);
        if href.is_some() {
            a.render_open(&mut cursor.buffer);
            img.render_closed(&mut cursor.buffer);
            a.render_close(&mut cursor.buffer);
        } else {
            img.render_closed(&mut cursor.buffer);
        }
        td.render_close(&mut cursor.buffer);
        tr.render_close(&mut cursor.buffer);
        tbody.render_close(&mut cursor.buffer);
        table.render_close(&mut cursor.buffer);
    }

    fn render_text(&self, href: &Option<String>, cursor: &mut RenderCursor) -> Result<(), Error> {
        let td = self.set_style_td_text(Tag::td());
        let wrapper = if href.is_some() {
            Tag::new("a")
                .maybe_add_attribute("href", href.clone())
                .maybe_add_attribute("rel", self.attribute("rel"))
                .maybe_add_attribute("target", self.attribute("target"))
        } else {
            Tag::new("span")
        };
        let wrapper = self.set_style_text(wrapper);

        td.render_open(&mut cursor.buffer);
        wrapper.render_open(&mut cursor.buffer);
        for child in self.element.children.iter() {
            let renderer = child.renderer(self.context());
            renderer.render(cursor)?;
        }
        wrapper.render_close(&mut cursor.buffer);
        td.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'element, 'header> Render<'element, 'header>
    for Renderer<'element, 'header, MjSocialElement, MjSocialElementExtra>
{
    fn default_attribute(&self, key: &str) -> Option<&'static str> {
        match key {
            "align" => Some("left"),
            "color" => Some("#000"),
            "border-radius" => Some("3px"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "line-height" => Some("1"),
            "padding" => Some("4px"),
            "target" => Some("_blank"),
            "text-decoration" => Some("none"),
            "text-padding" => Some("4px 4px 4px 0"),
            "vertical-align" => Some("middle"),
            _ => None,
        }
    }

    fn add_extra_attribute(&mut self, key: &str, value: &str) {
        self.extra
            .attributes
            .insert(key.to_string(), value.to_string());
    }

    fn raw_extra_attribute(&self, key: &str) -> Option<&str> {
        self.extra.attributes.get(key).map(|v| v.as_str())
    }

    fn raw_attribute(&self, key: &str) -> Option<&'element str> {
        self.element.attributes.get(key).map(|v| v.as_str())
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.extra.container_width = width;
    }

    fn context(&self) -> &'header RenderContext<'header> {
        self.context
    }

    fn render(&self, cursor: &mut RenderCursor) -> Result<(), Error> {
        let href = self.get_href();
        let tr = Tag::tr().maybe_add_class(self.attribute("css-class"));
        let td = self.set_style_td(Tag::td());

        tr.render_open(&mut cursor.buffer);
        td.render_open(&mut cursor.buffer);
        self.render_icon(&href, cursor);
        td.render_close(&mut cursor.buffer);
        if !self.element.children.is_empty() {
            self.render_text(&href, cursor)?;
        }
        tr.render_close(&mut cursor.buffer);
        Ok(())
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSocialElement {
    fn renderer(&'e self, context: &'h RenderContext<'h>) -> Box<dyn Render<'e, 'h> + 'r> {
        let extra = MjSocialElementExtra::new(
            self.attributes
                .get("name")
                .and_then(|name| SocialNetwork::find(name)),
        );
        Box::new(Renderer::new(context, self, extra))
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(render_ending_tag, "mj-social-element-ending");
}
