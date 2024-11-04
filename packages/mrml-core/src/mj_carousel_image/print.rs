#[cfg(test)]
mod tests {
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let mut item = crate::mj_carousel_image::MjCarouselImage::default();
        item.attributes
            .insert("src".to_string(), Some("http://localhost".into()));
        assert_eq!(
            "<mj-carousel-image src=\"http://localhost\" />",
            item.print_dense().unwrap()
        );
    }
}
