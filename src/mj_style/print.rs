use super::MJStyle;
use crate::prelude::print::{self, Print};
use crate::print_display;

impl Print for MJStyle {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        if pretty {
            print::indent(level, indent_size, self.print(false, level, indent_size))
        } else {
            format!("<{}>{}</{}>", super::NAME, self.children, super::NAME)
        }
    }
}

print_display!(MJStyle);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_style::MJStyle::from("Hello World!");
        assert_eq!("<mj-style>Hello World!</mj-style>", item.dense_print());
        assert_eq!("<mj-style>Hello World!</mj-style>\n", item.pretty_print());
    }
}
