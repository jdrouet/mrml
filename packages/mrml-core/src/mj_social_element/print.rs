use super::MJSocialElement;
use crate::print_attrs;

print_attrs!(MJSocialElement, super::NAME);

#[cfg(test)]
mod tests {
    use crate::prelude::print::Print;

    #[test]
    fn empty() {
        let mut item = crate::mj_social_element::MJSocialElement::default();
        item.attributes
            .insert("src".to_string(), "http://localhost".into());
        assert_eq!(
            "<mj-social-element src=\"http://localhost\" />",
            item.dense_print()
        );
    }
}
