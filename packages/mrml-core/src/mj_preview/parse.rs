#[cfg(test)]
mod tests {
    use crate::mj_preview::MjPreview;

    crate::should_sync_parse!(
        should_parse,
        MjPreview,
        "<mj-preview>Hello World!</mj-preview>"
    );
    crate::should_sync_parse!(should_parse_without_children, MjPreview, "<mj-preview />");
}
