use super::MJNavbarLink;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJNavbarLink {
    fn print(&self, pretty: bool, level: usize, indent_size: usize) -> String {
        print::open(
            super::NAME,
            Some(&self.attributes),
            true,
            pretty,
            level,
            indent_size,
        )
    }
}

impl fmt::Display for MJNavbarLink {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_navbar_link::MJNavbarLink::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-navbar-link src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
