use crate::prelude::print::{Printable, PrintableAttributes};

impl Printable for super::MjImage {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        printer.closed_tag();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_image::MjImage::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-image src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
