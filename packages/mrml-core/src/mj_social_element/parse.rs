#[cfg(test)]
mod tests {
    use crate::mj_social_element::MjSocialElement;

    crate::should_sync_parse!(
        parse_with_empty_children,
        MjSocialElement,
        r#"<mj-social-element name="facebook" />"#
    );

    crate::should_sync_parse!(
        parse_ending_tag,
        MjSocialElement,
        r#"<mj-social-element name="facebook">
    Share <b>test</b> hi
</mj-social-element>"#
    );
}
