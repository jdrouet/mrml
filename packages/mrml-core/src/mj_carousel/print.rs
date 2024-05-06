use crate::prelude::print::{PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableElement for super::MjCarousel {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }

    fn children(&self) -> &impl PrintableChildren {
        &self.children
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
