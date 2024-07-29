#[cfg(test)]
mod tests {
    use crate::mj_title::MjTitle;

    crate::should_sync_parse!(self_closing, MjTitle, "<mj-title />");
    crate::should_sync_parse!(normal, MjTitle, "<mj-title>Hello World!</mj-title>");
}
