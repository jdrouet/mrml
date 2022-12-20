use super::{MJImage, NAME};
use crate::print_attrs;

print_attrs!(MJImage, NAME);

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
