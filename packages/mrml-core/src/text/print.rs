use super::Text;
use crate::prelude::print::{Printable, Printer};

impl Printable for Text {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_str(self.0.as_str());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{prelude::print::Printable, text::Text};

    #[test]
    fn empty() {
        let item = Text::from("Hello World");
        assert_eq!("Hello World", item.print_dense().unwrap());
    }
}
