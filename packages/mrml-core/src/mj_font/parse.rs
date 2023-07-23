#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

    #[test]
    fn success() {
        assert!(Mjml::parse(
            r#"<mjml><mj-head><mj-font name="Comic" href="https://jolimail.io" /></mj-head></mjml>"#
        )
        .is_ok());
    }

    #[test]
    fn unexpected_attribute() {
        assert!(
            Mjml::parse(r#"<mjml><mj-head><mj-font unknown="whatever" /></mj-head></mjml>"#)
                .is_err()
        );
    }
}
