import mrml


def test_memory_loader():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.memory_loader(
            {
                "hello-world.mjml": "<mj-text>Hello World!</mj-text>",
            }
        )
    )
    result = mrml.to_html(
        '<mjml><mj-body><mj-include path="hello-world.mjml" /></mj-body></mjml>',
        parser_options=parser_options,
    )
    assert result.startswith("<!doctype html>")


def test_local_loader():
    parser_options = mrml.ParserOptions(
        include_loader=mrml.local_loader("./resources/partials")
    )
    result = mrml.to_html(
        '<mjml><mj-body><mj-include path="file:///hello-world.mjml" /></mj-body></mjml>',
        parser_options=parser_options,
    )
    assert result.startswith("<!doctype html>")
