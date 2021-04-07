use super::MJImage;
use crate::prelude::print::{self, Print};
use std::fmt;

impl Print for MJImage {
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

impl fmt::Display for MJImage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.dense_print().as_str())
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_image::MJImage::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!("<mj-image src=\"http://localhost\" />", item.dense_print());
    }
}
