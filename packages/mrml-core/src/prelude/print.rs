use std::fmt::{Debug, Display, Write};

// use crate::helper::sort::sort_by_key;
use crate::prelude::hash::Map;

pub(crate) trait PrintableAttributes {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result;
}

impl PrintableAttributes for Map<String, String> {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        for (name, value) in self.iter() {
            printer.push_attribute(name.as_str(), value.as_str())?;
        }
        Ok(())
    }
}

pub(crate) trait PrintableChildren {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result;
    fn has_children(&self) -> bool;
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
}

pub trait Printer {
    fn push_str(&mut self, value: &str);
    fn open_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result;
    fn push_attribute<N: Display + ?Sized, V: Debug + ?Sized>(
        &mut self,
        name: &N,
        value: &V,
    ) -> std::fmt::Result;
    fn close_tag(&mut self);
    fn closed_tag(&mut self) {
        self.push_str(" />");
    }
    fn end_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result;

    fn inner(self) -> String;
}

#[derive(Debug, Default)]
pub struct DensePrinter {
    buffer: String,
}

impl Printer for DensePrinter {
    #[inline]
    fn push_str(&mut self, value: &str) {
        self.buffer.push_str(value);
    }

    fn open_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
        write!(&mut self.buffer, "<{name}")
    }

    fn push_attribute<N: Display + ?Sized, V: Debug + ?Sized>(
        &mut self,
        name: &N,
        value: &V,
    ) -> std::fmt::Result {
        write!(&mut self.buffer, " {name}={value:?}")
    }

    fn close_tag(&mut self) {
        self.buffer.push('>');
    }

    fn end_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
        write!(&mut self.buffer, "</{name}>")
    }

    fn inner(self) -> String {
        self.buffer
    }
}

// #[derive(Debug)]
// pub struct PrettyPrinter {
//     indent_size: usize,
//     level: usize,
//     buffer: String,
// }

// impl Default for PrettyPrinter {
//     fn default() -> Self {
//         Self {
//             indent_size: 2,
//             level: 0,
//             buffer: String::default(),
//         }
//     }
// }

// impl Printer for PrettyPrinter {
//     #[inline]
//     fn push_str(&mut self, value: &str) {
//         self.buffer.push_str(value);
//     }

//     fn open_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
//         write!(&mut self.buffer, "<{name}")
//     }

//     fn push_attribute<N: Display + ?Sized, V: Debug + ?Sized>(
//         &mut self,
//         name: &N,
//         value: &V,
//     ) -> std::fmt::Result {
//         write!(&mut self.buffer, " {name}={value:?}")
//     }

//     fn close_tag(&mut self) {
//         self.buffer.push('>');
//     }

//     fn end_tag<N: Display + ?Sized>(&mut self, name: &N) -> std::fmt::Result {
//         write!(&mut self.buffer, "</{name}>")
//     }

//     fn inner(self) -> String {
//         self.buffer
//     }
// }

// pub trait Print {
//     fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String;

//     fn dense_print(&self) -> String {
//         self.print(false, 0, 2)
//     }

//     fn pretty_print(&self) -> String {
//         self.print(true, 0, 2)
//     }
// }

// pub fn indent(level: usize, indent_size: usize, value: String) -> String {
//     let spaces = level * indent_size;
//     let spaces = (0..spaces).map(|_| ' ').collect::<String>();
//     format!("{spaces}{value}\n")
// }

// pub fn attributes(attrs: Option<&Map<String, String>>) -> String {
//     attrs
//         .map(|attrs| {
//             let mut entries: Vec<(&String, &String)> = attrs.iter().collect();
//             entries.sort_by(sort_by_key);
//             entries
//                 .iter()
//                 .fold(String::default(), |mut res, (key, value)| {
//                     let _ = write!(res, " {key}=\"{value}\"");
//                     res
//                 })
//         })
//         .unwrap_or_default()
// }

// pub fn open(
//     tag: &str,
//     attrs: Option<&Map<String, String>>,
//     closed: bool,
//     pretty: bool,
//     level: usize,
//     indent_size: usize,
// ) -> String {
//     if pretty {
//         indent(
//             level,
//             indent_size,
//             open(tag, attrs, closed, false, level, indent_size),
//         )
//     } else if closed {
//         format!("<{}{} />", tag, attributes(attrs))
//     } else {
//         format!("<{}{}>", tag, attributes(attrs))
//     }
// }

// pub fn close(tag: &str, pretty: bool, level: usize, indent_size: usize) -> String {
//     if pretty {
//         indent(level, indent_size, close(tag, false, level, indent_size))
//     } else {
//         format!("</{tag}>")
//     }
// }
