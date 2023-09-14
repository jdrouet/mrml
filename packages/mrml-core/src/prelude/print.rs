use std::fmt::Write;

use crate::helper::sort::sort_by_key;
use crate::prelude::hash::Map;

pub trait Print {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String;

    fn dense_print(&self) -> String {
        self.print(false, 0, 2)
    }

    fn pretty_print(&self) -> String {
        self.print(true, 0, 2)
    }
}

pub fn indent(level: usize, indent_size: usize, value: String) -> String {
    let spaces = level * indent_size;
    let spaces = (0..spaces).map(|_| ' ').collect::<String>();
    format!("{spaces}{value}\n")
}

pub fn attributes(attrs: Option<&Map<String, String>>) -> String {
    attrs
        .map(|attrs| {
            let mut entries: Vec<(&String, &String)> = attrs.iter().collect();
            entries.sort_by(sort_by_key);
            entries
                .iter()
                .fold(String::default(), |mut res, (key, value)| {
                    let _ = write!(res, " {key}=\"{value}\"");
                    res
                })
        })
        .unwrap_or_default()
}

pub fn open(
    tag: &str,
    attrs: Option<&Map<String, String>>,
    closed: bool,
    pretty: bool,
    level: usize,
    indent_size: usize,
) -> String {
    if pretty {
        indent(
            level,
            indent_size,
            open(tag, attrs, closed, false, level, indent_size),
        )
    } else if closed {
        format!("<{}{} />", tag, attributes(attrs))
    } else {
        format!("<{}{}>", tag, attributes(attrs))
    }
}

pub fn close(tag: &str, pretty: bool, level: usize, indent_size: usize) -> String {
    if pretty {
        indent(level, indent_size, close(tag, false, level, indent_size))
    } else {
        format!("</{tag}>")
    }
}
