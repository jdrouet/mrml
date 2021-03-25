pub fn cleanup(input: &str) -> String {
    input
        .replace(" ", "")
        .replace("\n", "")
        .replace("<styletype=\"text/css\"></style>", "")
        .replace("style=\"\"", "")
        .replace("<div></div>", "")
}
