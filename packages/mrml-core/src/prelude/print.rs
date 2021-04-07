use crate::helper::sort::sort_by_key;
use std::collections::HashMap;

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
    format!("{}{}\n", spaces, value)
}

pub fn attributes(attrs: Option<&HashMap<String, String>>) -> String {
    attrs
        .map(|attrs| {
            let mut entries: Vec<(&String, &String)> = attrs.iter().collect();
            entries.sort_by(sort_by_key);
            entries
                .iter()
                .map(|(key, value)| format!(" {}=\"{}\"", key, value))
                .collect::<String>()
        })
        .unwrap_or_default()
}

pub fn open(
    tag: &str,
    attrs: Option<&HashMap<String, String>>,
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
    } else {
        let mut result = String::default();
        result.push('<');
        result.push_str(tag);
        result.push_str(&attributes(attrs));
        if closed {
            result.push_str(" />");
        } else {
            result.push('>');
        }
        result
    }
}

pub fn close(tag: &str, pretty: bool, level: usize, indent_size: usize) -> String {
    if pretty {
        indent(level, indent_size, close(tag, false, level, indent_size))
    } else {
        format!("</{}>", tag)
    }
}
