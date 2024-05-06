use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

impl Printable for super::MjCarousel {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        if self.children.is_empty() {
            printer.closed_tag();
        } else {
            printer.close_tag();
            self.children.print(printer)?;
            printer.end_tag(super::NAME)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_carousel::MjCarousel::default();
        assert_eq!("<mj-carousel />", item.print_dense().unwrap());
    }

    //     #[cfg(feature = "parse")]
    //     #[test]
    //     fn with_images() {
    //         let json = r#"<mjml>
    //   <mj-body>
    //     <mj-carousel>
    //       <mj-carousel-image />
    //     </mj-carousel>
    //   </mj-body>
    // </mjml>
    // "#;
    //         let root = crate::mjml::Mjml::parse(json).unwrap();
    //         assert_eq!(json, root.pretty_print());
    //     }
}
