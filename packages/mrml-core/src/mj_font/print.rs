use super::MJFont;
use crate::prelude::print::{self, Print};
use std::collections::HashMap;
use std::fmt;

impl Print for MJFont {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        format!(
            "<{} name=\"{}\" href=\"{}\" />",
            super::NAME,
            self.name,
            self.href
        );
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), self.name.clone());
        attrs.insert("href".to_string(), self.href.clone());
        print::open(super::NAME, Some(&attrs), true, pretty, level, indent_size)
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
