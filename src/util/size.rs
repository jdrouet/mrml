pub fn parse_pixel(input: String) -> String {
    if input.ends_with("px") {
        input.get(0..(input.len() - 2)).unwrap().to_string()
    } else {
        input
    }
}
