#[cfg(test)]
mod tests {
    use crate::mj_wrapper::MjWrapper;

    crate::should_sync_parse!(
        parse_br_element,
        MjWrapper,
        "<mj-wrapper><h1>hello</h1><br><h2>world</h2></mj-wrapper>"
    );
}
