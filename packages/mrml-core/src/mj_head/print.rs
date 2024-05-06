use crate::prelude::print::{PrintableChildren, PrintableElement};

impl PrintableElement for super::MjHead {
    fn tag(&self) -> &str {
        super::NAME
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
        let item = crate::mj_head::MjHead::default();
        assert_eq!("<mj-head />", item.print_dense().unwrap());
    }

    //     #[cfg(feature = "parse")]
    //     #[test]
    //     fn with_all() {
    //         let origin = r#"<mjml>
    //   <mj-head>
    //     <mj-attributes>
    //       <mj-all font-size="12px" />
    //     </mj-attributes>
    //     <mj-breakpoint width="12px" />
    //     <mj-font href="https://jolimail.io" name="Comic" />
    //     <mj-preview>Hello World with all!</mj-preview>
    //     <mj-title>Hello World!</mj-title>
    //   </mj-head>
    // </mjml>
    // "#;
    //         let root = crate::mjml::Mjml::parse(origin).unwrap();
    //         assert_eq!(origin, root.pretty_print());
    //         let head = root.head().unwrap();
    //         assert_eq!(head.breakpoint().unwrap().value(), "12px");
    //         assert_eq!(head.preview().unwrap().content(), "Hello World with all!");
    //         assert_eq!(head.title().unwrap().content(), "Hello World!");
    //         assert_eq!(head.children().len(), 5);
    //     }

    #[test]
    fn with_title() {
        let mut item = crate::mj_head::MjHead::default();
        item.children.push(crate::mj_head::MjHeadChild::MjTitle(
            crate::mj_title::MjTitle::from("Hello World!"),
        ));
        assert_eq!(
            "<mj-head><mj-title>Hello World!</mj-title></mj-head>",
            item.print_dense().unwrap()
        );
    }
}
