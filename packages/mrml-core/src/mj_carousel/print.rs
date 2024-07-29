#[cfg(test)]
mod tests {
    use crate::mj_carousel::MjCarousel;
    use crate::mj_carousel_image::MjCarouselImage;
    use crate::prelude::print::Printable;

    #[test]
    fn empty() {
        let item = crate::mj_carousel::MjCarousel::default();
        assert_eq!("<mj-carousel />", item.print_dense().unwrap());
    }

    #[test]
    fn with_images() {
        let item = MjCarousel::new(Default::default(), vec![MjCarouselImage::default().into()]);
        assert_eq!(
            r#"<mj-carousel>
  <mj-carousel-image />
</mj-carousel>
"#,
            item.print_pretty().unwrap()
        );
    }
}
