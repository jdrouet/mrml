#[cfg(test)]
mod tests {
    use crate::mj_accordion_text::MjAccordionText;

    crate::should_sync_parse!(
        should_work_with_child_text,
        MjAccordionText,
        "<mj-accordion-text>Hello</mj-accordion-text>"
    );

    crate::should_sync_parse!(
        should_work_with_no_children,
        MjAccordionText,
        "<mj-accordion-text />"
    );

    crate::should_not_sync_parse!(
        should_error_with_no_closing,
        MjAccordionText,
        "<mj-accordion-text>",
        "EndOfStream"
    );
}
