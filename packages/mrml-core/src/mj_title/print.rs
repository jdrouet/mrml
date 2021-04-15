use super::MJTitle;
use crate::prelude::print::{self, Print};
use crate::print_display;

impl Print for MJTitle {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        if pretty {
            print::indent(level, indent_size, self.print(false, level, indent_size))
        } else {
            format!("<{}>{}</{}>", super::NAME, self.children, super::NAME)
        }
    }
}

print_display!(MJTitle);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn normal() {
        let item = crate::mj_title::MJTitle::from("Hello World!");
        assert_eq!("<mj-title>Hello World!</mj-title>", item.dense_print());
        assert_eq!("<mj-title>Hello World!</mj-title>\n", item.pretty_print());
    }
}
