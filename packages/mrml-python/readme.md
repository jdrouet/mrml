# mrml-python

This project is a reimplementation of the nice `MJML` markup language in Rust, built for python.

To have more information, take a look at [the repository](https://github.com/jdrouet/mrml).

## Usage in python

```python
import mrml

# without options
result = mrml.to_html("<mjml></mjml>")
assert result.startswith("<!doctype html>")

# with options
parser_options = mrml.ParserOptions(include_loader = mrml.memory_loader({
    'hello-world.mjml': '<mj-text>Hello World!</mj-text>',
}))
result = mrml.to_html("<mjml><mj-body><mj-include path=\"hello-world.mjml\" /></mj-body></mjml>", parser_options = parser_options)
assert result.startswith("<!doctype html>")
```
