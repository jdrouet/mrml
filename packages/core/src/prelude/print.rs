use std::collections::HashMap;

pub trait Print {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize);

    fn dense_print(&self) -> String {
        let mut res = String::default();
        self.print(&mut res, false, 0, 2);
        res
    }

    fn pretty_print(&self) -> String {
        let mut res = String::default();
        self.print(&mut res, true, 0, 2);
        res
    }
}

pub fn print_indent(f: &mut String, level: usize, indent_size: usize) {
    let spaces = level * indent_size;
    for _ in 0..spaces {
        f.push_str(" ");
    }
}

pub fn print_open(
    f: &mut String,
    tag: &str,
    attrs: Option<&HashMap<String, String>>,
    closed: bool,
    pretty: bool,
    level: usize,
    indent_size: usize,
) {
    if pretty {
        print_indent(f, level, indent_size);
    }
    f.push_str("<");
    f.push_str(tag);
    if let Some(attrs) = attrs {
        for (key, value) in attrs.iter() {
            f.push_str(" ");
            f.push_str(key);
            f.push_str("=\"");
            f.push_str(value);
            f.push_str("\"");
        }
    }
    if closed {
        f.push_str(" />");
    } else {
        f.push_str(">");
    }
    if pretty {
        f.push_str("\n");
    }
}

pub fn print_close(f: &mut String, tag: &str, pretty: bool, level: usize, indent_size: usize) {
    if pretty {
        print_indent(f, level, indent_size);
    }
    f.push_str("</");
    f.push_str(tag);
    f.push_str(">");
    if pretty {
        f.push_str("\n");
    }
}
