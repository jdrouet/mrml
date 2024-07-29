#[cfg(test)]
mod tests {
    use crate::mj_button::MjButton;

    crate::should_parse!(
        success,
        MjButton,
        r#"<mj-button>
    <!-- Just a comment -->
    <b>foo</b>
    bar
</mj-button>"#
    );

    crate::should_async_parse!(
        async_success,
        MjButton,
        r#"<mj-button>
    <!-- Just a comment -->
    <b>foo</b>
    bar
</mj-button>"#
    );
}
