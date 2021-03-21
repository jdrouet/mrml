use super::MJFont;
use crate::prelude::print::{print_open, Print};
use std::collections::HashMap;
use std::fmt;

impl Print for MJFont {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        let mut attrs = HashMap::<String, String>::new();
        attrs.insert("name".to_string(), self.name.clone());
        attrs.insert("href".to_string(), self.href.clone());
        print_open(
            f,
            super::NAME,
            Some(&attrs),
            true,
            pretty,
            level,
            indent_size,
        );
        if pretty {
            f.push_str("\n");
        }
    }
}

impl fmt::Display for MJFont {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_font::MJFont {
            name: String::from("Comic sans MS"),
            href: String::from("http://localhost"),
        };
        assert_eq!(
            "<mj-font href=\"http://localhost\" name=\"Comic sans MS\" />",
            item.dense_print()
        );
    }
}
