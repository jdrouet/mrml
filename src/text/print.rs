use super::Text;
use crate::prelude::print::{self, Print};
use crate::print_display;

impl Print for Text {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        if pretty {
            print::indent(level, indent_size, self.0.clone())
        } else {
            self.0.clone()
        }
    }
}

print_display!(Text);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::text::Text::from("Hello World");
        assert_eq!("Hello World", item.dense_print());
    }
}
