use std::fmt::{Debug, Display, Write};

// use crate::helper::sort::sort_by_key;
use crate::prelude::hash::Map;

pub trait PrintableAttributes {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result;
}

impl PrintableAttributes for () {
    fn print<P: Printer>(&self, _: &mut P) -> std::fmt::Result {
        Ok(())
    }
}

impl PrintableAttributes for Map<String, String> {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        for (name, value) in self.iter() {
            printer.push_attribute(name.as_str(), value.as_str())?;
        }
        Ok(())
    }
}

pub trait PrintableChildren {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result;
    fn has_children(&self) -> bool;
}

impl PrintableChildren for () {
    fn has_children(&self) -> bool {
        false
    }

    fn print<P: Printer>(&self, _: &mut P) -> std::fmt::Result {
        Ok(())
    }
}

impl<C: Printable> PrintableChildren for Vec<C> {
    fn has_children(&self) -> bool {
        !self.is_empty()
    }

    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        for item in self.iter() {
            item.print(printer)?;
        }
        Ok(())
    }
}

use crate::mj_accordion::MjAccordionChild;
use crate::mj_attributes::MjAttributesChild;
use crate::mj_body::MjBodyChild;
use crate::mj_carousel::MjCarouselChild;
use crate::mj_head::MjHeadChild;
use crate::mj_navbar::MjNavbarChild;
use crate::mj_raw::MjRawChild;
use crate::mj_social::MjSocialChild;

use crate::comment::Comment;
use crate::mj_accordion::MjAccordion;
use crate::mj_accordion_element::MjAccordionElement;
use crate::mj_attributes::MjAttributes;
use crate::mj_attributes_all::MjAttributesAll;
use crate::mj_attributes_class::MjAttributesClass;
use crate::mj_attributes_element::MjAttributesElement;
use crate::mj_breakpoint::MjBreakpoint;
use crate::mj_button::MjButton;
use crate::mj_carousel::MjCarousel;
use crate::mj_carousel_image::MjCarouselImage;
use crate::mj_column::MjColumn;
use crate::mj_divider::MjDivider;
use crate::mj_font::MjFont;
use crate::mj_group::MjGroup;
use crate::mj_hero::MjHero;
use crate::mj_image::MjImage;
use crate::mj_include::body::MjIncludeBody;
use crate::mj_include::head::MjIncludeHead;
use crate::mj_navbar::MjNavbar;
use crate::mj_navbar_link::MjNavbarLink;
use crate::mj_preview::MjPreview;
use crate::mj_raw::MjRaw;
use crate::mj_section::MjSection;
use crate::mj_social::MjSocial;
use crate::mj_social_element::MjSocialElement;
use crate::mj_spacer::MjSpacer;
use crate::mj_style::MjStyle;
use crate::mj_table::MjTable;
use crate::mj_text::MjText;
use crate::mj_title::MjTitle;
use crate::mj_wrapper::MjWrapper;
use crate::node::Node;
use crate::text::Text;

#[enum_dispatch::enum_dispatch(
    MjAccordionChild,
    MjAttributesChild,
    MjBodyChild,
    MjCarouselChild,
    MjHeadChild,
    MjNavbarChild,
    MjRawChild,
    MjSocialChild
)]
pub trait Printable {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result;

    fn print_dense(&self) -> Result<String, std::fmt::Error> {
        let mut p = DensePrinter::default();
        self.print(&mut p)?;
        Ok(p.inner())
    }

    fn print_pretty(&self) -> Result<String, std::fmt::Error> {
        let mut p = PrettyPrinter::default();
        self.print(&mut p)?;
        Ok(p.inner())
    }
}

pub trait PrintableElement {
    fn tag(&self) -> &str;
    fn attributes(&self) -> &impl PrintableAttributes {
        &()
    }
    fn children(&self) -> &impl PrintableChildren {
        &()
    }
}

impl<E: PrintableElement> Printable for E {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        let tag = self.tag();
        let attrs = self.attributes();
        let children = self.children();

        printer.push_indent();
        printer.open_tag(tag)?;
        attrs.print(printer)?;
        if children.has_children() {
            printer.close_tag();
            printer.push_new_line();
            printer.increase_indent();
            children.print(printer)?;
            printer.decrease_indent();
            printer.push_indent();
            printer.end_tag(tag)?;
        } else {
            printer.closed_tag();
        }
        printer.push_new_line();
        Ok(())
    }
}

pub trait Printer {
    fn push_new_line(&mut self);
    fn push_indent(&mut self);
    fn increase_indent(&mut self);
    fn decrease_indent(&mut self);

    fn push(&mut self, value: char);
    fn push_str(&mut self, value: &str);

    fn open_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result;
    fn close_tag(&mut self) {
        self.push('>');
    }
    fn closed_tag(&mut self) {
        self.push_str(" />");
    }
    fn end_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result;

    fn push_attribute<N: Display + ?Sized, V: Debug + ?Sized>(
        &mut self,
        name: &N,
        value: &V,
    ) -> std::fmt::Result;

    fn inner(self) -> String;
}

#[derive(Debug, Default)]
pub struct DensePrinter {
    buffer: String,
}

impl Printer for DensePrinter {
    #[inline]
    fn push_new_line(&mut self) {}
    #[inline]
    fn push_indent(&mut self) {}
    #[inline]
    fn increase_indent(&mut self) {}
    #[inline]
    fn decrease_indent(&mut self) {}

    #[inline]
    fn push(&mut self, value: char) {
        self.buffer.push(value);
    }

    #[inline]
    fn push_str(&mut self, value: &str) {
        self.buffer.push_str(value);
    }

    #[inline]
    fn open_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
        write!(&mut self.buffer, "<{name}")
    }

    #[inline]
    fn push_attribute<N: Display + ?Sized, V: Debug + ?Sized>(
        &mut self,
        name: &N,
        value: &V,
    ) -> std::fmt::Result {
        write!(&mut self.buffer, " {name}={value:?}")
    }

    #[inline]
    fn end_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
        write!(&mut self.buffer, "</{name}>")
    }

    #[inline]
    fn inner(self) -> String {
        self.buffer
    }
}

#[derive(Debug)]
pub struct PrettyPrinter {
    indent_size: usize,
    level: usize,
    buffer: String,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {
            indent_size: 2,
            level: 0,
            buffer: String::default(),
        }
    }
}

impl Printer for PrettyPrinter {
    #[inline]
    fn push_new_line(&mut self) {
        self.buffer.push('\n');
    }

    #[inline]
    fn push_indent(&mut self) {
        self.buffer
            .extend(std::iter::repeat(' ').take(self.level * self.indent_size));
    }

    #[inline]
    fn increase_indent(&mut self) {
        self.level += 1;
    }

    #[inline]
    fn decrease_indent(&mut self) {
        self.level -= 1;
    }

    #[inline]
    fn push(&mut self, value: char) {
        self.buffer.push(value);
    }

    #[inline]
    fn push_str(&mut self, value: &str) {
        self.buffer.push_str(value);
    }

    #[inline]
    fn open_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
        write!(&mut self.buffer, "<{name}")
    }

    #[inline]
    fn push_attribute<N: Display + ?Sized, V: Debug + ?Sized>(
        &mut self,
        name: &N,
        value: &V,
    ) -> std::fmt::Result {
        write!(&mut self.buffer, " {name}={value:?}")
    }

    #[inline]
    fn end_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
        write!(&mut self.buffer, "</{name}>")
    }

    #[inline]
    fn inner(self) -> String {
        self.buffer
    }
}
