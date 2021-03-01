use super::MJNavbar;
use crate::elements::body::prelude::{
    to_children_iterator, BodyChild, BodyComponent, BodyComponentChildIterator,
};
use crate::elements::error::Error;
use crate::elements::prelude::*;
use crate::util::attributes::*;
use crate::util::condition::*;
use crate::util::context::Context;
use crate::util::header::Header;
use crate::util::size::Size;
use crate::util::tag::Tag;

impl MJNavbar {
    fn set_style_input(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none !important")
            .set_style("max-height", "0")
            .set_style("visibility", "hidden")
    }

    fn set_style_label(&self, tag: Tag) -> Tag {
        tag.set_style("display", "block")
            .set_style("cursor", "pointer")
            .set_style("mso-hide", "all")
            .set_style("-moz-user-select", "none")
            .set_style("user-select", "none")
            .maybe_set_style("color", self.get_attribute("ico-color"))
            .maybe_set_style("font-size", self.get_attribute("ico-font-size"))
            .maybe_set_style("font-family", self.get_attribute("ico-font-family"))
            .maybe_set_style("text-transform", self.get_attribute("ico-text-transform"))
            .maybe_set_style("text-decoration", self.get_attribute("ico-text-decoration"))
            .maybe_set_style("line-height", self.get_attribute("ico-line-height"))
            .maybe_set_style("padding-top", self.get_attribute("ico-padding-top"))
            .maybe_set_style("padding-right", self.get_attribute("ico-padding-right"))
            .maybe_set_style("padding-bottom", self.get_attribute("ico-padding-bottom"))
            .maybe_set_style("padding-left", self.get_attribute("ico-padding-left"))
            .maybe_set_style("padding", self.get_attribute("ico-padding"))
    }

    fn set_style_trigger(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("max-height", "0px")
            .set_style("max-width", "0px")
            .set_style("font-size", "0px")
            .set_style("overflow", "hidden")
    }

    fn set_style_ico_close(&self, tag: Tag) -> Tag {
        tag.set_style("display", "none")
            .set_style("mso-hide", "all")
    }

    fn set_style_ico_open(&self, tag: Tag) -> Tag {
        tag.set_style("mso-hide", "all")
    }

    fn has_hamburger(&self) -> bool {
        self.get_attribute("hamburger")
            .and_then(|value| {
                if value == "hamburger" {
                    Some(true)
                } else {
                    None
                }
            })
            .is_some()
    }

    fn render_hamburger(&self, _header: &Header) -> String {
        let input = self
            .set_style_input(Tag::new("input"))
            .set_class("mj-menu-checkbox")
            .set_attribute("id", self.id.as_str())
            .set_attribute("type", "checkbox");
        let div = self
            .set_style_trigger(Tag::div())
            .set_class("mj-menu-trigger");
        let label = self
            .set_style_label(Tag::new("label"))
            .maybe_set_attribute("align", self.get_attribute("ico-align"))
            .set_class("mj-menu-label")
            .set_attribute("for", self.id.as_str());
        let span_open = self
            .set_style_ico_open(Tag::new("span"))
            .set_class("mj-menu-icon-open");
        let span_close = self
            .set_style_ico_close(Tag::new("span"))
            .set_class("mj-menu-icon-close");
        let content = span_open
            .render(self.get_attribute("ico-open").unwrap_or(&String::default()))
            + &span_close.render(
                self.get_attribute("ico-close")
                    .unwrap_or(&String::default()),
            );
        let content = div.render(label.render(content));
        mso_negation_conditional_tag(input.closed()) + &content
    }
}

impl Component for MJNavbar {
    fn update_header(&self, header: &mut Header) {
        header.add_style(format!(r#"
        noinput.mj-menu-checkbox {{ display:block!important; max-height:none!important; visibility:visible!important; }}
        @media only screen and (max-width:{}) {{
          .mj-menu-checkbox[type="checkbox"] ~ .mj-inline-links {{ display:none!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-inline-links,
          .mj-menu-checkbox[type="checkbox"] ~ .mj-menu-trigger {{ display:block!important; max-width:none!important; max-height:none!important; font-size:inherit!important; }}
          .mj-menu-checkbox[type="checkbox"] ~ .mj-inline-links > a {{ display:block!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-menu-trigger .mj-menu-icon-close {{ display:block!important; }}
          .mj-menu-checkbox[type="checkbox"]:checked ~ .mj-menu-trigger .mj-menu-icon-open {{ display:none!important; }}
        }}
        "#, header.breakpoint.to_string()));
        self.get_children().for_each(|child| {
            child.update_header(header);
        });
    }

    fn context(&self) -> Option<&Context> {
        self.context.as_ref()
    }

    fn set_context(&mut self, ctx: Context) {
        self.context = Some(ctx);
        let child_base = Context::new(
            self.get_container_width(),
            self.get_siblings(),
            self.get_raw_siblings(),
            0,
        );
        self.children
            .iter_mut()
            .enumerate()
            .for_each(|(idx, child)| {
                child
                    .inner_mut()
                    .set_context(child_base.clone().set_index(idx));
            });
    }

    fn render(&self, header: &Header) -> Result<String, Error> {
        let div = Tag::div().set_class("mj-inline-links");
        let table =
            Tag::table_presentation().maybe_set_attribute("align", self.get_attribute("align"));
        let tr = Tag::tr();
        let content = self
            .get_children()
            .try_fold(String::default(), |res, child| {
                Ok(res + &child.render(header)?)
            })?;
        let before = conditional_tag(table.open() + &tr.open());
        let after = conditional_tag(tr.close() + &table.close());
        let content = div.render(before + &content + &after);
        if self.has_hamburger() {
            Ok(self.render_hamburger(header) + &content)
        } else {
            Ok(content)
        }
    }
}

impl BodyComponent for MJNavbar {
    fn attributes(&self) -> Option<&Attributes> {
        Some(&self.attributes)
    }

    fn get_children(&self) -> BodyComponentChildIterator {
        to_children_iterator(&self.children)
    }

    fn get_children_len(&self) -> usize {
        self.children.len()
    }

    fn get_current_width(&self) -> Option<Size> {
        self.context().and_then(|ctx| ctx.container_width())
    }

    fn set_style(&self, _name: &str, tag: Tag) -> Tag {
        tag
    }
}
