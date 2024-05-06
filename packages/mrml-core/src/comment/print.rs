use crate::prelude::print::Printable;

impl Printable for super::Comment {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_indent();
        printer.push_str("<!--");
        printer.push_str(self.children.as_str());
        printer.push_str("-->");
        printer.push_new_line();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::comment::Comment::from("Hello World");
        assert_eq!("<!--Hello World-->", item.print_dense().unwrap());
    }
}
