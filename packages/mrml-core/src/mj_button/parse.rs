#[cfg(test)]
mod tests {
    #[test]
    fn success() {
        crate::mjml::Mjml::parse(
            r#"<mjml>
    <mj-body>
        <mj-button>
            <!-- Just a comment -->
            <b>foo</b>
            bar
        </mj-button>
    </mj-body>
</mjml>"#,
        )
        .unwrap();
    }
}
