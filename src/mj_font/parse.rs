#[cfg(test)]
mod tests {
    use crate::mjml::MJML;

    #[test]
    fn success() {
        assert!(MJML::parse(
            r#"<mjml><mj-head><mj-font name="Comic" href="https://jolimail.io" /></mj-head></mjml>"#
        )
        .is_ok());
    }

    #[test]
    fn unexpected_attribute() {
        assert!(
            MJML::parse(r#"<mjml><mj-head><mj-font unknown="whatever" /></mj-head></mjml>"#)
                .is_err()
        );
    }
}
