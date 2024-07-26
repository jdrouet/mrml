use crate::prelude::print::PrintableAttributes;

impl PrintableAttributes for super::MjFontAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("name", self.name.as_str())?;
        printer.push_attribute("href", self.href.as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_font::MjFont;
    use crate::prelude::print::Printable;

    #[test]
    fn normal() {
        let item = MjFont::build("Comic sans MS", "http://localhost");
        assert_eq!(
            "<mj-font name=\"Comic sans MS\" href=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
