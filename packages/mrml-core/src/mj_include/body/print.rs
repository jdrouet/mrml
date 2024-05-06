use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableAttributes for super::MjIncludeBodyAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("path", self.path.as_str())?;
        if !self.kind.is_default() {
            printer.push_attribute("type", self.kind.as_ref())?;
        }
        Ok(())
    }
}

impl PrintableElement for super::MjIncludeBody {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;
    use crate::mj_include::body::{MjIncludeBody, MjIncludeBodyChild, MjIncludeBodyKind};
    use crate::prelude::print::Printable;

    #[test]
    fn kind_string() {
        assert_eq!(MjIncludeBodyKind::Html.as_ref(), "html");
        assert_eq!(MjIncludeBodyKind::Mjml.as_ref(), "mjml");
    }

    #[test]
    fn simple() {
        let mut elt = MjIncludeBody::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeBodyChild::MjButton(MjButton::default())];
        assert_eq!(
            elt.print_dense().unwrap(),
            "<mj-include path=\"memory:include.mjml\" />"
        );
    }

    #[test]
    fn html_kind() {
        let mut elt = MjIncludeBody::default();
        elt.attributes.kind = MjIncludeBodyKind::Html;
        elt.attributes.path = "memory:include.html".to_string();
        elt.children = vec![MjIncludeBodyChild::MjButton(MjButton::default())];
        assert_eq!(
            elt.print_dense().unwrap(),
            "<mj-include path=\"memory:include.html\" type=\"html\" />"
        );
    }
}
