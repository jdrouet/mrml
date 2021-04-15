use super::MJFont;
use crate::prelude::print::{self, Print};
use crate::print_display;
use std::collections::HashMap;

impl Print for MJFont {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        let mut attrs = HashMap::new();
        attrs.insert("name".to_string(), self.name().to_string());
        attrs.insert("href".to_string(), self.href().to_string());
        print::open(super::NAME, Some(&attrs), true, pretty, level, indent_size)
    }
}

print_display!(MJFont);

#[cfg(test)]
mod tests {
    use crate::mj_font::{MJFont, MJFontAttributes};
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = MJFont {
            attributes: MJFontAttributes {
                name: String::from("Comic sans MS"),
                href: String::from("http://localhost"),
            },
        };
        assert_eq!(
            "<mj-font href=\"http://localhost\" name=\"Comic sans MS\" />",
            item.dense_print()
        );
    }
}
