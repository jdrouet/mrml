#[cfg(test)]
mod tests {
    use crate::mj_attributes_all::MjAttributesAll;

    crate::should_sync_parse!(parse_complete, MjAttributesAll, r#"<mj-all color="red" />"#);
}
