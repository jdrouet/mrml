use super::MjIncludeHeadKind;

use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableElement for super::MjIncludeHead {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }
}

impl PrintableAttributes for super::MjIncludeHeadAttributes {
    fn print<P: crate::prelude::print::Printer>(&self, printer: &mut P) -> std::fmt::Result {
        printer.push_attribute("path", self.path.as_str())?;
        match self.kind {
            MjIncludeHeadKind::Html => {
                printer.push_attribute("type", "html")?;
            }
            MjIncludeHeadKind::Css { inline } => {
                printer.push_attribute("type", "css")?;
                if inline {
                    printer.push_attribute("css-inline", "inline")?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_include::head::{MjIncludeHead, MjIncludeHeadChild, MjIncludeHeadKind};
    use crate::mj_title::MjTitle;
    use crate::prelude::print::Printable;

    #[test]
    fn simple() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.path = "memory:include.mjml".to_string();
        elt.children = vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
            "Hello World!".to_owned(),
        ))];
        assert_eq!(
            elt.print_dense().unwrap(),
            "<mj-include path=\"memory:include.mjml\" />"
        );
    }

    #[test]
    fn html_kind() {
        let mut elt = MjIncludeHead::default();
        elt.attributes.kind = MjIncludeHeadKind::Html;
        elt.attributes.path = "memory:include.html".to_string();
        elt.children = vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
            "Hello World!".to_owned(),
        ))];
        assert_eq!(
            elt.print_dense().unwrap(),
            "<mj-include path=\"memory:include.html\" type=\"html\" />"
        );
    }
}
