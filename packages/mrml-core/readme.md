# MRML

[![Crates.io](https://img.shields.io/crates/d/mrml)](https://crates.io/crates/mrml)
![Crates.io](https://img.shields.io/crates/v/mrml)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core?ref=badge_shield)

[![.github/workflows/main.yml](https://github.com/jolimail/mrml-core/actions/workflows/main.yml/badge.svg)](https://github.com/jolimail/mrml-core/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/jolimail/mrml-core/branch/main/graph/badge.svg?token=SIOPR0YWZA)](https://codecov.io/gh/jolimail/mrml-core)

[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/jolimail/mrml-core.svg)](http://isitmaintained.com/project/jolimail/mrml-core "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/jolimail/mrml-core.svg)](http://isitmaintained.com/project/jdrouet/mrml "Percentage of issues still open")
[![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jolimail/mrml-core/maintainability)

This project is a reimplementation of the nice [MJML](https://mjml.io/) markup language in Rust.

# How to use?

To use it you can simply update your `Cargo.toml` by adding
```toml
[dependencies]
mrml = { version = "*" }
serde = { version = "1", features = ["derive"] }
```

And you can then just create a `main.rs` with the following code
```rust
# #[cfg(feature = "parse")]
# {
let root = mrml::parse("<mjml><mj-body></mj-body></mjml>").expect("parse template");
let opts = mrml::prelude::render::RenderOptions::default();
match root.element.render(&opts) {
    Ok(content) => println!("{}", content),
    Err(_) => println!("couldn't render mjml template"),
};
# }
```

## Using `mj-include`

You can also use the `mj-include` component by specifying a
[loader](crate::prelude::parser).

```rust
# #[cfg(feature = "parse")]
# {
use mrml::prelude::parser::ParserOptions;
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;

let loader = MemoryIncludeLoader::from(vec![("partial.mjml", "<mj-button>Hello</mj-button>")]);
let options = ParserOptions {
    include_loader: Box::new(loader),
};
match mrml::parse_with_options("<mjml><mj-head /><mj-body><mj-include path=\"partial.mjml\" /></mj-body></mjml>", &options) {
    Ok(_) => println!("Success!"),
    Err(err) => eprintln!("Something went wrong: {err:?}"),
}
# }
```

## Using `mj-include` with an async loader

If you want to use the async version to fetch the includes, you've to enable
the `async` feature and the required loaders (`http-loader-async-reqwest` in
this example).

```rust
# #[cfg(all(feature = "parse", feature = "render", feature = "async", feature = "local-loader", feature = "http-loader", feature = "http-loader-async-reqwest"))]
# tokio_test::block_on(async {
use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::local_loader::LocalIncludeLoader;
use mrml::prelude::parser::multi_loader::MultiIncludeLoader;
use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
use mrml::prelude::parser::loader::AsyncIncludeLoader;
use mrml::prelude::parser::AsyncParserOptions;
use mrml::prelude::render::RenderOptions;
use std::path::PathBuf;
use std::sync::Arc;

let resolver = MultiIncludeLoader::<Box<dyn AsyncIncludeLoader + Send + Sync + 'static>>::new()
    .with_starts_with("memory://", Box::new(MemoryIncludeLoader::from(vec![("basic.mjml", "<mj-button>Hello</mj-button>")])))
    .with_starts_with("file://", Box::new(LocalIncludeLoader::new(PathBuf::default().join("resources").join("compare").join("success"))))
    .with_starts_with("https://", Box::new(HttpIncludeLoader::<AsyncReqwestFetcher>::allow_all()))
    .with_any(Box::<NoopIncludeLoader>::default());
let parser_options = AsyncParserOptions {
    include_loader: Box::new(resolver),
};
let render_options = RenderOptions::default();
let json = r#"<mjml>
<mj-body>
<mj-include path="file://basic.mjml" />
<mj-include path="memory://basic.mjml" />
</mj-body>
</mjml>"#;
match mrml::async_parse_with_options(json, Arc::new(parser_options)).await {
    Ok(mjml) => match mjml.element.render(&render_options) {
        Ok(html) => println!("{html}"),
        Err(err) => eprintln!("Couldn't render template: {err:?}"),
    },
    Err(err) => eprintln!("Couldn't parse template: {err:?}"),
}
# })
```

## Using `mrml` in Python

This crate can also be used in Python. The crate is available with pypi and
you can find some documentation [here](https://pypi.org/project/mrml/).

```python
import mrml

# without options
result = mrml.to_html("<mjml></mjml>")
assert result.content.startswith("<!doctype html>")

# with options
parser_options = mrml.ParserOptions(include_loader = mrml.memory_loader({
    'hello-world.mjml': '<mj-text>Hello World!</mj-text>',
}))
result = mrml.to_html("<mjml><mj-body><mj-include path=\"hello-world.mjml\" /></mj-body></mjml>", parser_options = parser_options)
assert result.content.startswith("<!doctype html>")
```

# Why?

A Node.js server rendering an MJML template takes around **20 MB** of RAM at
startup and **130 MB** under stress test. In Rust, it takes less than **1.7 MB** at
startup and a bit less that **3 MB** under stress test. The Rust version can
also handle twice as many requests per second. You can perform the
benchmarks by running `bash script/run-bench.sh`.

Also, the JavaScript implementation cannot run in the browser; the Rust
one (and WebAssembly one) can.

## You want to contribute?

Feel free to read our [contributing](./contributing.md) section and the [code of conduct](./code-of-conduct.md).

## Performance

With the same Linux amd64 machine, to render the amario template

- Node: 606.59ms
- Rust: 3.48ms

## Missing implementations

- `mj-style[inline]`: not yet implemented. It requires parsing the generated html to apply the inline styles afterward (that's how it's done in mjml) which would kill the performances. Applying it at render time would improve the performance but it would still require to parse the CSS.
- `mj-include`: not yet implemented. It requires to handle loading remote templates when using mrml in a wasm (browser or server side) format, which implies being able to load from a different location (`file://`, `https://`, relative, etc).

## Who is using MRML?

[<img src="https://www.blizzstatic.com/www/marketing/images/logo.svg" height="22px" />](https://www.blizzfull.com/)

<i>If you are using MRML and want to be added to this list, don't hesitate to create an issue or open a pull request.</i>

## What is using MRML?

- [mjml_nif](https://github.com/adoptoposs/mjml_nif) - Elixir library
- [mrml-ruby](https://github.com/hardpixel/mrml-ruby) - Ruby library
- [mjml-python](https://github.com/mgd020/mjml-python) - Python library
- [wagtail-newsletter](https://github.com/wagtail/wagtail-newsletter) - Wagtail extension
- [intellij-mjml-support](https://github.com/timo-reymann/intellij-mjml-support) - MJML support for the IntelliJ Platform
- [mjml-php](https://github.com/alekitto/mjml-php) - PHP extension

<i>If you are using MRML and want to be added to this list, don't hesitate to create an issue or open a pull request.</i>

## You want to sponsor us?

[<img src="https://liberapay.com/assets/liberapay/icon-v2_white-on-yellow.svg?etag=.Z1LYSBJ8Z6GWUeLUUEf2XA~~" height="35px" />](https://liberapay.com/jdrouet/)
[<img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" height="35px" />](https://www.buymeacoffee.com/jdrouet)

<i>Thanks to [zachzurn](https://github.com/zachzurn).</i>

## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core?ref=badge_large)
