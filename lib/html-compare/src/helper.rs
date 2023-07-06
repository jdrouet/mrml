pub fn cleanup_text(input: &str) -> String {
    input.replace([' ', '\t', '\n'], "")
}
