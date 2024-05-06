use crate::prelude::print::{Printable, PrintableAttributes, PrintableChildren};

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

impl Printable for super::Mjml {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.open_tag(super::NAME)?;
        self.attributes.print(printer)?;
        if self.children.body.is_none() && self.children.head.is_none() {
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
                body: Some(crate::mj_body::MjBody::default()),
            },
        };
        assert_eq!("<mjml><mj-body /></mjml>", item.print_dense().unwrap());
    }
}
