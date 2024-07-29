#[cfg(test)]
mod tests {
    use crate::mj_text::MjText;

    crate::should_parse!(self_closing, MjText, "<mj-text />");
    crate::should_parse!(normal, MjText, "<mj-text>Hello World!</mj-text>");
}
