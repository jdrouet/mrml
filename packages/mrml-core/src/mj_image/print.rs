#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_image::MjImage::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-image src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
        assert_eq!(
            "<mj-image src=\"http://localhost\" />\n",
            item.print_pretty().unwrap()
        );
    }
}
