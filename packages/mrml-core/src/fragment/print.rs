use super::Fragment;
use crate::prelude::print::{Printable, PrintableChildren, Printer};

impl<T: Printable> Printable for Fragment<T> {
    fn print<P: Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.open_tag("")?;
        printer.close_tag();
        if !self.children.is_empty() {
            printer.push_new_line();
            printer.increase_indent();
            self.children.print(printer)?;
            printer.decrease_indent();
            printer.push_indent();
            printer.push_new_line();
        }
        printer.end_tag("")?;
        Ok(())
    }
}
