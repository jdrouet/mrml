use crate::prelude::print::{PrintableAttributes, PrintableChildren, PrintableElement};

impl PrintableElement for super::MjCarousel {
    fn tag(&self) -> &str {
        super::NAME
    }

    fn attributes(&self) -> &impl PrintableAttributes {
        &self.attributes
    }

    fn children(&self) -> &impl PrintableChildren {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mj_carousel::MjCarousel, mj_carousel_image::MjCarouselImage, prelude::print::Printable,
    };

    #[test]
    fn empty() {
        let item = crate::mj_carousel::MjCarousel::default();
        assert_eq!("<mj-carousel />", item.print_dense().unwrap());
    }

    #[test]
    fn with_images() {
        let item = MjCarousel {
            attributes: Default::default(),
            children: vec![MjCarouselImage {
                attributes: Default::default(),
            }
            .into()],
        };
        assert_eq!(
            r#"<mj-carousel>
  <mj-carousel-image />
</mj-carousel>
"#,
            item.print_pretty().unwrap()
        );
    }
}
