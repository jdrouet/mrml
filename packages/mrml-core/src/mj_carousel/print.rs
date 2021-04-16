use super::{MJCarousel, NAME};
use crate::print_attrs_children;

print_attrs_children!(MJCarousel, NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_carousel::MJCarousel::default();
        assert_eq!("<mj-carousel></mj-carousel>", item.dense_print());
    }

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
        let root = crate::mjml::MJML::parse(json).unwrap();
        assert_eq!(json, root.pretty_print());
    }
}
