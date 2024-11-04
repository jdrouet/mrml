use std::fmt::{Debug, Display, Write};
use std::marker::PhantomData;

use crate::prelude::hash::Map;

pub(crate) trait PrintableAttributes {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result;
}

impl PrintableAttributes for () {
    fn print<P: Printer>(&self, _: &mut P) -> std::fmt::Result {
        Ok(())
    }
}

impl PrintableAttributes for Map<String, Option<String>> {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        for (name, value) in self.iter() {
            printer.push_attribute(name.as_str(), value.as_deref())?;
        }
        Ok(())
    }
}

pub(crate) trait PrintableChildren {
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

impl<T: StaticTag, A: PrintableAttributes, C: PrintableChildren> PrintableElement
    for super::Component<PhantomData<T>, A, C>
{
    type Attrs = A;
    type Children = C;

    fn tag(&self) -> &str {
        T::static_tag()
    }

    fn attributes(&self) -> &Self::Attrs {
        &self.attributes
    }

    fn children(&self) -> &Self::Children {
        &self.children
    }
}

use super::StaticTag;
use crate::comment::Comment;
use crate::mj_accordion::{MjAccordion, MjAccordionChild};
use crate::mj_accordion_element::MjAccordionElement;
use crate::mj_attributes::{MjAttributes, MjAttributesChild};
use crate::mj_attributes_all::MjAttributesAll;
use crate::mj_attributes_class::MjAttributesClass;
use crate::mj_attributes_element::MjAttributesElement;
use crate::mj_body::MjBodyChild;
use crate::mj_breakpoint::MjBreakpoint;
use crate::mj_button::MjButton;
use crate::mj_carousel::{MjCarousel, MjCarouselChild};
use crate::mj_carousel_image::MjCarouselImage;
use crate::mj_column::MjColumn;
use crate::mj_divider::MjDivider;
use crate::mj_font::MjFont;
use crate::mj_group::MjGroup;
use crate::mj_head::MjHeadChild;
use crate::mj_hero::MjHero;
use crate::mj_image::MjImage;
use crate::mj_include::body::MjIncludeBody;
use crate::mj_include::head::MjIncludeHead;
use crate::mj_navbar::{MjNavbar, MjNavbarChild};
use crate::mj_navbar_link::MjNavbarLink;
use crate::mj_preview::MjPreview;
use crate::mj_raw::{MjRaw, MjRawChild};
use crate::mj_section::MjSection;
use crate::mj_social::{MjSocial, MjSocialChild};
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

pub(crate) trait PrintableElement {
    type Attrs: PrintableAttributes;
    type Children: PrintableChildren;

    fn tag(&self) -> &str;
    fn attributes(&self) -> &Self::Attrs;
    fn children(&self) -> &Self::Children;
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

pub struct PrintAttribute<N, V> {
    name: N,
    value: V,
}

impl<N, V> From<(N, V)> for PrintAttribute<N, V> {
    fn from((name, value): (N, V)) -> Self {
        Self { name, value }
    }
}

impl<'a> std::fmt::Display for PrintAttribute<&'a str, &'a str> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={:?}", self.name, self.value)
    }
}

impl<'a> std::fmt::Display for PrintAttribute<&'a str, Option<&'a str>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.value {
            Some(ref value) => write!(f, "{}={:?}", self.name, value),
            None => write!(f, "{}", self.name),
        }
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

    fn push_attribute<N, V>(&mut self, name: N, value: V) -> std::fmt::Result
    where
        PrintAttribute<N, V>: Display;

    fn inner(self) -> String;
}

#[derive(Debug, Default)]
pub(crate) struct DensePrinter {
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
    fn push_attribute<N, V>(&mut self, name: N, value: V) -> std::fmt::Result
    where
        PrintAttribute<N, V>: Display,
    {
        write!(&mut self.buffer, " {}", PrintAttribute::from((name, value)))
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
pub(crate) struct PrettyPrinter {
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
    fn push_attribute<N, V>(&mut self, name: N, value: V) -> std::fmt::Result
    where
        PrintAttribute<N, V>: Display,
    {
        write!(&mut self.buffer, " {}", PrintAttribute::from((name, value)))
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
