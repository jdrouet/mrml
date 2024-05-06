use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableAttributes for super::MjmlAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        if let Some(ref item) = self.dir {
            printer.push_attribute("dir", item.as_str())?;
        }
        if let Some(ref item) = self.lang {
            printer.push_attribute("lang", item.as_str())?;
        }
        if let Some(ref item) = self.owa {
            printer.push_attribute("owa", item.as_str())?;
        }
        Ok(())
    }
}

impl PrintableChildren for super::MjmlChildren {
    fn has_children(&self) -> bool {
        self.body.is_some() || self.head.is_some()
    }

    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        if let Some(ref item) = self.head {
            item.print(printer)?;
        }
        if let Some(ref item) = self.body {
            item.print(printer)?;
        }
        Ok(())
    }
}

impl PrintableElement for super::Mjml {
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
    use crate::comment::Comment;
    use crate::mj_body::MjBody;
    use crate::mj_column::MjColumn;
    use crate::mj_section::MjSection;
    use crate::mj_text::MjText;
    use crate::mjml::{Mjml, MjmlChildren};
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = Mjml::default();
        assert_eq!("<mjml />", item.print_dense().unwrap());
    }

    #[test]
    fn with_lang() {
        let mut item = Mjml::default();
        item.attributes.lang = Some("fr".to_string());
        assert_eq!("<mjml lang=\"fr\" />", item.print_dense().unwrap());
    }

    #[test]
    fn with_body() {
        let item = Mjml {
            attributes: Default::default(),
            children: MjmlChildren {
                head: None,
                body: Some(MjBody {
                    attributes: Default::default(),
                    children: vec![
                        MjSection {
                            attributes: Default::default(),
                            children: vec![MjColumn {
                                attributes: Default::default(),
                                children: vec![MjText {
                                    attributes: Default::default(),
                                    children: vec![],
                                }
                                .into()],
                            }
                            .into()],
                        }
                        .into(),
                        Comment {
                            children: "Hello World!".into(),
                        }
                        .into(),
                    ],
                }),
            },
        };
        assert_eq!("<mjml><mj-body><mj-section><mj-column><mj-text /></mj-column></mj-section><!--Hello World!--></mj-body></mjml>", item.print_dense().unwrap());
        assert_eq!(
            r#"<mjml>
  <mj-body>
    <mj-section>
      <mj-column>
        <mj-text />
      </mj-column>
    </mj-section>
    <!--Hello World!-->
  </mj-body>
</mjml>
"#,
            item.print_pretty().unwrap()
        );
    }
}
