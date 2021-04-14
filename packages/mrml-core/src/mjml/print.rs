use super::MJML;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJML {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(super::NAME, None, false, pretty, level, indent_size)
            + &self
                .head()
                .as_ref()
                .map(|h| h.print(pretty, level + 1, indent_size))
                .unwrap_or_default()
            + &self
                .body()
                .as_ref()
                .map(|b| b.print(pretty, level + 1, indent_size))
                .unwrap_or_default()
            + &print::close(super::NAME, pretty, level, indent_size)
    }
}

impl fmt::Display for MJML {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::mjml::{MJMLChildren, MJML};
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = MJML::default();
        assert_eq!("<mjml></mjml>", format!("{}", item));
    }

    #[test]
    fn with_body() {
        let item = MJML {
            children: MJMLChildren {
                head: None,
                body: Some(crate::mj_body::MJBody::default()),
            },
        };
        assert_eq!("<mjml><mj-body></mj-body></mjml>", item.dense_print());
        assert_eq!(
            "<mjml>\n  <mj-body>\n  </mj-body>\n</mjml>\n",
            item.pretty_print()
        );
    }
}
