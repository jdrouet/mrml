use super::MJSocial;
use crate::print_attrs_children;

print_attrs_children!(MJSocial, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let item = crate::mj_social::MJSocial::default();
        assert_eq!("<mj-social></mj-social>", item.dense_print());
    }
}
