use super::MjIncludeHeadKind;
use crate::prelude::print::{PrintableAttributes, PrintableElement};

impl PrintableElement for super::MjIncludeHead {
    type Attrs = super::MjIncludeHeadAttributes;
    type Children = ();

    fn tag(&self) -> &str {
        crate::mj_include::NAME
    }

    fn attributes(&self) -> &Self::Attrs {
        &self.0.attributes
    }

    fn children(&self) -> &Self::Children {
        &()
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
    use crate::mj_include::head::{
        MjIncludeHead, MjIncludeHeadAttributes, MjIncludeHeadChild, MjIncludeHeadKind,
    };
    use crate::mj_title::MjTitle;
    use crate::prelude::print::Printable;

    #[test]
    fn simple() {
        let elt = MjIncludeHead::new(
            MjIncludeHeadAttributes::new("memory:include.mjml"),
            vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
                "Hello World!".to_owned(),
            ))],
        );
        assert_eq!(
            elt.print_dense().unwrap(),
            "<mj-include path=\"memory:include.mjml\" />"
        );
    }

    #[test]
    fn html_kind() {
        let elt = MjIncludeHead::new(
            MjIncludeHeadAttributes::new("memory:include.html").with_kind(MjIncludeHeadKind::Html),
            vec![MjIncludeHeadChild::MjTitle(MjTitle::from(
                "Hello World!".to_owned(),
            ))],
        );
        assert_eq!(
            elt.print_dense().unwrap(),
            "<mj-include path=\"memory:include.html\" type=\"html\" />"
        );
    }
}
