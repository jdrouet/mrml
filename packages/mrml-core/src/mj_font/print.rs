use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableAttributes for super::MjFontAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("name", self.name.as_str())?;
        printer.push_attribute("href", self.href.as_str())
    }
}

impl PrintableElement for super::MjFont {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_font::{MjFont, MjFontAttributes};
    use crate::prelude::print::Printable;

    #[test]
    fn normal() {
        let item = MjFont {
            attributes: MjFontAttributes {
                name: String::from("Comic sans MS"),
                href: String::from("http://localhost"),
            },
        };
        assert_eq!(
            "<mj-font name=\"Comic sans MS\" href=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
