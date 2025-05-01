use super::ConditionalComment;
use crate::prelude::print::{Printable, Printer};

impl Printable for ConditionalComment {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.push_str(self.0.as_str());
        printer.push_new_line();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::conditional_comment::ConditionalComment;
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = ConditionalComment::from("<!--[if mso]>");
        assert_eq!("<!--[if mso]>", item.print_dense().unwrap());
    }
}
