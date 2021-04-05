use super::MJML;
use crate::prelude::print::{print_close, print_open, Print};
use std::fmt;

impl Print for MJML {
    fn print(&self, f: &mut String, pretty: bool, level: usize, indent_size: usize) {
        print_open(f, super::NAME, None, false, pretty, level, indent_size);
        if let Some(head) = self.head.as_ref() {
            head.print(f, pretty, level + 1, indent_size);
        }
        if let Some(body) = self.body.as_ref() {
            body.print(f, pretty, level + 1, indent_size);
        }
        print_close(f, super::NAME, pretty, level, indent_size);
    }
}

impl fmt::Display for MJML {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::MJML;
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = MJML::default();
        assert_eq!("<mjml></mjml>", format!("{}", item));
    }

    #[test]
    fn with_body() {
        let item = MJML {
            body: Some(crate::mj_body::MJBody::default()),
            ..MJML::default()
        };
        assert_eq!("<mjml><mj-body></mj-body></mjml>", item.dense_print());
        assert_eq!(
            "<mjml>\n  <mj-body>\n  </mj-body>\n</mjml>\n",
            item.pretty_print()
        );
    }
}
