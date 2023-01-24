#[cfg(test)]
mod tests {
    use crate::mjml::Mjml;

    #[test]
    fn parse_ending_tag() {
        let template = r#"
        <mjml>
          <mj-body>
            <mj-social>
              <mj-social-element name="facebook">
                Share <b>test</b> hi
              </mj-social-element>
            </mj-social>
          </mj-body>
        </mjml>
        "#;
        Mjml::parse(template).unwrap();
    }
}
