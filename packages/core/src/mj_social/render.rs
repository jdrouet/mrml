use super::{MJSocial, NAME};
use crate::helper::condition::conditional_tag;
use crate::helper::size::{Pixel, Size};
use crate::helper::tag::Tag;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

const EXTRA_CONTAINER_KEY: [&str; 13] = [
    "inner-padding",
    "border-radius",
    "color",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "icon-size",
    "icon-height",
    "icon-padding",
    "text-padding",
    "line-height",
    "text-decoration",
];
const EXTRA_CHILD_KEY: [&str; 13] = [
    "padding",
    "border-radius",
    "color",
    "font-family",
    "font-size",
    "font-weight",
    "font-style",
    "icon-size",
    "icon-height",
    "icon-padding",
    "text-padding",
    "line-height",
    "text-decoration",
];

struct MJSocialRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJSocial,
    container_width: Option<Pixel>,
    siblings: usize,
    raw_siblings: usize,
}

impl<'e, 'h> MJSocialRender<'e, 'h> {
    fn set_style_table_vertical(&self, tag: Tag) -> Tag {
        tag.add_style("margin", "0px")
    }

    fn is_horizontal(&self) -> bool {
        self.attribute("mode")
            .map(|mode| mode == "horizontal")
            .unwrap_or(true)
    }

    fn build_child_attributes(&self) -> Vec<(&str, String)> {
        EXTRA_CONTAINER_KEY
            .iter()
            .zip(EXTRA_CHILD_KEY.iter())
            .filter_map(|(con_key, child_key)| {
                if let Some(value) = self.attribute(con_key) {
                    Some((*child_key, value))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn render_horizontal(&self, opts: &Options) -> Result<String, Error> {
        let table = Tag::table_presentation().maybe_add_attribute("align", self.attribute("align"));
        let tr = Tag::tr();
        let td = Tag::td();
        let inner_table = Tag::table_presentation()
            .maybe_add_attribute("align", self.attribute("align"))
            .add_style("display", "inline-table")
            .add_style("float", "none");
        let before = conditional_tag(table.open() + &tr.open());
        let after = conditional_tag(tr.close() + &table.close());
        let child_attributes = self.build_child_attributes();
        let content = self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                child_attributes.iter().for_each(|(key, value)| {
                    renderer.add_extra_attribute(key, &value);
                });
                Ok(res
                    + &conditional_tag(td.open())
                    + &inner_table.render(renderer.render(opts)?)
                    + &conditional_tag(td.close()))
            },
        )?;
        Ok(before + &content + &after)
    }

    fn render_vertical(&self, opts: &Options) -> Result<String, Error> {
        let table = self.set_style_table_vertical(Tag::table_presentation());
        let child_attributes = self.build_child_attributes();
        let content = self.element.children.iter().enumerate().try_fold(
            String::default(),
            |res, (index, child)| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_index(index);
                child_attributes.iter().for_each(|(key, value)| {
                    renderer.add_extra_attribute(key, &value);
                });
                Ok(res + &renderer.render(opts)?)
            },
        )?;
        Ok(table.render(content))
    }
}

impl<'e, 'h> Render<'h> for MJSocialRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "align" => Some("center"),
            "border-radius" => Some("3px"),
            "color" => Some("#333333"),
            "font-family" => Some("Ubuntu, Helvetica, Arial, sans-serif"),
            "font-size" => Some("13px"),
            "icon-size" => Some("20px"),
            "line-height" => Some("22px"),
            "mode" => Some("horizontal"),
            "padding" => Some("10px 25px"),
            "text-decoration" => Some("none"),
            _ => None,
        }
    }

    fn attributes(&self) -> Option<&HashMap<String, String>> {
        Some(&self.element.attributes)
    }

    fn tag(&self) -> Option<&str> {
        Some(NAME)
    }

    fn header(&self) -> Ref<Header<'h>> {
        self.header.borrow()
    }

    fn get_width(&self) -> Option<Size> {
        self.container_width
            .as_ref()
            .map(|w| Size::Pixel(w.clone()))
    }

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn set_siblings(&mut self, value: usize) {
        self.siblings = value;
    }

    fn set_raw_siblings(&mut self, value: usize) {
        self.raw_siblings = value;
    }

    fn render(&self, opts: &Options) -> Result<String, Error> {
        let font_families = self.attribute("font-family").unwrap_or_default(); // never happens
        self.header.borrow_mut().add_font_families(font_families);
        if self.is_horizontal() {
            self.render_horizontal(opts)
        } else {
            self.render_vertical(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJSocial {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJSocialRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
            siblings: 1,
            raw_siblings: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::helper::test::compare;
    use crate::mjml::MJML;
    use crate::prelude::render::Options;

    #[test]
    fn basic() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn align() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-align.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-align.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn border_radius() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-border-radius.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-border-radius.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn class() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-class.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-class.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn color() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-color.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-color.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn container_background_color() {
        let opts = Options::default();
        let template = include_str!(
            "../../resources/compare/success/mj-social-container-background-color.mjml"
        );
        let expected = include_str!(
            "../../resources/compare/success/mj-social-container-background-color.html"
        );
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font_family() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-font-family.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-font-family.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn font() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-font.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-font.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn icon() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-icon.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-icon.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn link() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-link.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-link.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn mode() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-mode.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-mode.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn padding() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-padding.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-padding.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn text() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-social-text.mjml");
        let expected = include_str!("../../resources/compare/success/mj-social-text.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }
}
