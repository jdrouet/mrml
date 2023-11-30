#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_carousel::MjCarousel::default();
        assert_eq!("<mj-carousel />", item.dense_print());
    }

    #[cfg(feature = "parse")]
    #[test]
    fn with_images() {
        let json = r#"<mjml>
  <mj-body>
    <mj-carousel>
      <mj-carousel-image />
    </mj-carousel>
  </mj-body>
</mjml>
"#;
        let root = crate::mjml::Mjml::parse(json).unwrap();
        assert_eq!(json, root.pretty_print());
    }
}
