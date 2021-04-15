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
}
