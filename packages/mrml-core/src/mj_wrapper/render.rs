use super::{MJWrapper, NAME};
use crate::helper::condition::{conditional_tag, END_CONDITIONAL_TAG, START_CONDITIONAL_TAG};
use crate::helper::size::Pixel;
use crate::helper::tag::Tag;
use crate::mj_section::WithMJSectionBackground;
use crate::prelude::render::{Error, Header, Options, Render, Renderable};
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

struct MJWrapperRender<'e, 'h> {
    header: Rc<RefCell<Header<'h>>>,
    element: &'e MJWrapper,
    container_width: Option<Pixel>,
}

impl<'e, 'h> WithMJSectionBackground<'h> for MJWrapperRender<'e, 'h> {}

impl<'e, 'h> MJWrapperRender<'e, 'h> {
    fn current_width(&self) -> Option<Pixel> {
        self.container_width.as_ref().map(|width| {
            let hborder = self.get_border_horizontal();
            let hpadding = self.get_padding_horizontal();
            Pixel::new(width.value() - hborder.value() - hpadding.value())
        })
    }

    fn is_full_width(&self) -> bool {
        self.attribute_exists("full-width")
    }

    fn render_with_background<T: AsRef<str>>(&self, content: T) -> String {
        let full_width = self.is_full_width();
        let vrect = Tag::new("v:rect")
            .maybe_add_attribute(
                "mso-width-percent",
                if full_width { Some("1000") } else { None },
            )
            .maybe_add_style(
                "width",
                if full_width {
                    None
                } else {
                    self.container_width.as_ref().map(|v| v.to_string())
                },
            )
            .add_attribute("xmlns:v", "urn:schemas-microsoft-com:vml")
            .add_attribute("fill", "true")
            .add_attribute("stroke", "false");
        let vfill = self.get_vfill_tag();
        let vtextbox = Tag::new("v:textbox")
            .add_attribute("inset", "0,0,0,0")
            .add_style("mso-fit-shape-to-text", "true");
        let before = conditional_tag(vrect.open() + &vfill.closed() + &vtextbox.open());
        let after = conditional_tag(vtextbox.close() + &vrect.close());
        before + content.as_ref() + &after
    }

    fn set_style_section_div(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.add_style("margin", "0px auto")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
            .maybe_add_style(
                "max-width",
                self.container_width.as_ref().map(|item| item.to_string()),
            )
    }

    fn render_wrap<T: AsRef<str>>(&self, content: T) -> String {
        let table = Tag::table_borderless()
            .add_attribute("align", "center")
            .maybe_add_attribute(
                "width",
                self.container_width.as_ref().map(|p| p.value().to_string()),
            )
            .maybe_add_style(
                "width",
                self.container_width.as_ref().map(|v| v.to_string()),
            )
            .maybe_add_suffixed_class(self.attribute("css-class"), "outlook");
        let tr = Tag::tr();
        let td = Tag::td()
            .add_style("line-height", "0px")
            .add_style("font-size", "0px")
            .add_style("mso-line-height-rule", "exactly");
        let before = conditional_tag(table.open() + &tr.open() + &td.open());
        let after = conditional_tag(td.close() + &tr.close() + &table.close());
        before + content.as_ref() + &after
    }

    fn get_siblings(&self) -> usize {
        self.element.children.len()
    }

    fn get_raw_siblings(&self) -> usize {
        self.element
            .children
            .iter()
            .filter(|elt| elt.is_raw())
            .count()
    }

    fn render_wrapped_children(&self, opts: &Options) -> Result<String, Error> {
        let tr = Tag::tr();
        let siblings = self.get_siblings();
        let raw_siblings = self.get_raw_siblings();
        let current_width = self.current_width();
        let container_width = self.container_width.as_ref().map(|v| v.to_string());
        let content = self
            .element
            .children
            .iter()
            .try_fold(String::default(), |res, child| {
                let mut renderer = child.renderer(Rc::clone(&self.header));
                renderer.set_siblings(siblings);
                renderer.set_raw_siblings(raw_siblings);
                renderer.set_container_width(current_width.clone());
                if child.is_raw() {
                    Ok(res + END_CONDITIONAL_TAG + &renderer.render(opts)? + START_CONDITIONAL_TAG)
                } else {
                    let td = renderer
                        .set_style("td-outlook", Tag::td())
                        .maybe_add_attribute("align", renderer.attribute("align"))
                        .maybe_add_attribute("width", container_width.as_ref())
                        .maybe_add_suffixed_class(renderer.attribute("css-class"), "outlook");
                    Ok(res
                        + &td.open()
                        + END_CONDITIONAL_TAG
                        + &renderer.render(opts)?
                        + START_CONDITIONAL_TAG
                        + &td.close())
                }
            })?;
        Ok(tr.render(content))
    }

    fn set_style_section_inner_div(&self, tag: Tag) -> Tag {
        tag.add_style("line-height", "0")
            .add_style("font-size", "0")
    }

    fn set_style_section_table(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            tag
        } else {
            self.set_background_style(tag)
        };
        base.add_style("width", "100%")
            .maybe_add_style("border-radius", self.attribute("border-radius"))
    }

    fn set_style_section_td(&self, tag: Tag) -> Tag {
        tag.maybe_add_style("border", self.attribute("border"))
            .maybe_add_style("border-bottom", self.attribute("border-bottom"))
            .maybe_add_style("border-left", self.attribute("border-left"))
            .maybe_add_style("border-right", self.attribute("border-right"))
            .maybe_add_style("border-top", self.attribute("border-top"))
            .maybe_add_style("direction", self.attribute("direction"))
            .add_style("font-size", "0px")
            .maybe_add_style("padding", self.attribute("padding"))
            .maybe_add_style("padding-bottom", self.attribute("padding-bottom"))
            .maybe_add_style("padding-left", self.attribute("padding-left"))
            .maybe_add_style("padding-right", self.attribute("padding-right"))
            .maybe_add_style("padding-top", self.attribute("padding-top"))
            .maybe_add_style("text-align", self.attribute("text-align"))
    }

    fn render_section(&self, opts: &Options) -> Result<String, Error> {
        let is_full_width = self.is_full_width();
        let div = self
            .set_style_section_div(Tag::div())
            .maybe_add_class(if is_full_width {
                None
            } else {
                self.attribute("css-class")
            });
        let inner_div = self.set_style_section_inner_div(Tag::div());
        let table = self.set_style_section_table(
            Tag::table_presentation()
                .add_attribute("align", "center")
                .maybe_add_attribute(
                    "background",
                    if is_full_width {
                        None
                    } else {
                        self.attribute("background-url")
                    },
                ),
        );
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = self.set_style_section_td(Tag::td());
        let inner_table = Tag::table_presentation();
        let content = conditional_tag(inner_table.render(self.render_wrapped_children(opts)?));
        let content = table.render(tbody.render(tr.render(td.render(content))));
        Ok(div.render(if self.has_background() {
            inner_div.render(content)
        } else {
            content
        }))
    }

    fn set_style_table_full_width(&self, tag: Tag) -> Tag {
        let base = if self.is_full_width() {
            self.set_background_style(tag)
        } else {
            tag
        };
        base.maybe_add_style("border-radius", self.attribute("border-radius"))
            .add_style("width", "100%")
    }

    fn get_full_width_table(&self) -> Tag {
        self.set_style_table_full_width(Tag::table_presentation())
            .add_attribute("align", "center")
            .maybe_add_class(self.attribute("css-class"))
            .maybe_add_attribute("background", self.attribute("background-url"))
    }

    fn render_full_width(&self, opts: &Options) -> Result<String, Error> {
        let table = self.get_full_width_table();
        let tbody = Tag::tbody();
        let tr = Tag::tr();
        let td = Tag::td();
        let content = self.render_wrap(self.render_section(opts)?);
        let content = if self.has_background() {
            self.render_with_background(content)
        } else {
            content
        };
        Ok(table.render(tbody.render(tr.render(td.render(content)))))
    }

    fn render_simple(&self, opts: &Options) -> Result<String, Error> {
        let section = self.render_section(opts)?;

        let section = if self.has_background() {
            self.render_with_background(section)
        } else {
            section
        };
        Ok(self.render_wrap(section))
    }
}

impl<'e, 'h> Render<'h> for MJWrapperRender<'e, 'h> {
    fn default_attribute(&self, name: &str) -> Option<&str> {
        match name {
            "background-position" => Some("top center"),
            "background-repeat" => Some("repeat"),
            "background-size" => Some("auto"),
            "direction" => Some("ltr"),
            "padding" => Some("20px 0"),
            "text-align" => Some("center"),
            "text-padding" => Some("4px 4px 4px 0"),
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

    fn set_container_width(&mut self, width: Option<Pixel>) {
        self.container_width = width;
    }

    fn render(&self, opts: &Options) -> Result<String, Error> {
        if self.is_full_width() {
            self.render_full_width(opts)
        } else {
            self.render_simple(opts)
        }
    }
}

impl<'r, 'e: 'r, 'h: 'r> Renderable<'r, 'e, 'h> for MJWrapper {
    fn renderer(&'e self, header: Rc<RefCell<Header<'h>>>) -> Box<dyn Render<'h> + 'r> {
        Box::new(MJWrapperRender::<'e, 'h> {
            element: self,
            header,
            container_width: None,
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
        let template = include_str!("../../resources/compare/success/mj-wrapper.mjml");
        let expected = include_str!("../../resources/compare/success/mj-wrapper.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn background() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-wrapper-background.mjml");
        let expected = include_str!("../../resources/compare/success/mj-wrapper-background.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn border() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-wrapper-border.mjml");
        let expected = include_str!("../../resources/compare/success/mj-wrapper-border.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn other() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-wrapper-other.mjml");
        let expected = include_str!("../../resources/compare/success/mj-wrapper-other.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }

    #[test]
    fn padding() {
        let opts = Options::default();
        let template = include_str!("../../resources/compare/success/mj-wrapper-padding.mjml");
        let expected = include_str!("../../resources/compare/success/mj-wrapper-padding.html");
        let root = MJML::parse(template.to_string()).unwrap();
        let result = root.render(&opts).unwrap();
        compare(expected, result.as_str());
    }
}
