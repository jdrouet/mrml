use std::cell::{Ref, RefCell};
use std::rc::Rc;

use super::network::SocialNetwork;
use super::{MjSocialElement, NAME};
use crate::helper::size::{Pixel, Size};
use crate::helper::tag::Tag;
use crate::prelude::hash::Map;
use crate::prelude::render::{Error, Header, Render, RenderOptions, Renderable};

const DEFAULT_ICON_ORIGIN: &str = "https://www.mailjet.com/images/theme/v1/icons/ico-social/";

struct MjSocialElementRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MjSocialElement,
    extra: Map<String, String>,
    container_width: Option<Pixel>,
    network: Option<SocialNetwork>,
}

impl<'e, 'h> MjSocialElementRender<'e, 'h> {
    fn get_background_color(&self) -> Option<String> {
        self.attribute("background-color").or_else(|| {
            self.network
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

    fn get_icon_src(&self, opts: &RenderOptions) -> Option<String> {
        self.attribute("src").or_else(|| {
            self.network.as_ref().map(|net| {
                if let Some(ref origin) = opts.social_icon_origin {
                    net.icon_src(origin)
                } else {
                    net.icon_src(DEFAULT_ICON_ORIGIN)
                }
            })
        })
    }

    fn set_style_img(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("display", "block")
    }

    fn set_style_icon(&self, tag: Tag) -> Tag {
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

    fn set_style_table(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("background", self.get_background_color())
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .maybe_add_style("width", self.get_icon_size().map(|size| size.to_string()))
    }

    fn set_style_td(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("vertical-align", self.attribute("vertical-align"))
    }

    fn set_style_td_text(&self, tag: Tag) -> Tag {
        tag.add_style("vertical-align", "middle")
            .maybe_add_style("padding", self.attribute("text-padding"))
    }

    fn set_style_text(&self, tag: Tag) -> Tag {
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
                self.network
                    .as_ref()
                    .and_then(|net| net.share_url(&href))
                    .or(Some(href))
            })
            .unwrap_or_default()
    }

    fn render_icon(&self, href: &Option<String>, opts: &RenderOptions) -> String {
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
            .maybe_add_attribute("src", self.get_icon_src(opts))
            .maybe_add_attribute(
                "width",
                self.get_icon_size().map(|size| size.value().to_string()),
            );

        table.render(tbody.render(tr.render(td.render(if href.is_some() {
            a.render(img.closed())
        } else {
            img.closed()
        }))))
    }

    fn render_text(&self, href: &Option<String>, opts: &RenderOptions) -> Result<String, Error> {
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
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let renderer = child.renderer(Rc::clone(&self.header));
                Ok(res + &renderer.render(opts)?)
            })?;
        Ok(td.render(wrapper.render(content)))
    }
}

impl<'e, 'h> Render<'h> for MjSocialElementRender<'e, 'h> {
    fn default_attribute(&self, key: &str) -> Option<&str> {
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
        self.extra.insert(key.to_string(), value.to_string());
    }

    fn extra_attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.extra)
    }

    fn attributes(&self) -> Option<&Map<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn render(&self, opts: &RenderOptions) -> Result<String, Error> {
        let href = self.get_href();
        let tr = Tag::tr().maybe_add_class(self.attribute("css-class"));
        let td = self.set_style_td(Tag::td());

        let mut res = td.render(self.render_icon(&href, opts));
        if !self.element.children.is_empty() {
            res.push_str(&self.render_text(&href, opts)?);
        }
        Ok(tr.render(res))
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MjSocialElement {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MjSocialElementRender::<'e, 'h> {
            element: self,
            header,
            extra: Map::new(),
            container_width: None,
            network: self
                .attributes
                .get("name")
                .and_then(|name| SocialNetwork::find(name)),
        })
    }
}

#[cfg(test)]
mod tests {
    crate::should_render!(render_ending_tag, "mj-social-element-ending");
}
