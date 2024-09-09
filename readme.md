# MRML

[![Crates.io](https://img.shields.io/crates/d/mrml)](https://crates.io/crates/mrml)
![Crates.io](https://img.shields.io/crates/v/mrml)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjdrouet%2Fmrml.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjdrouet%2Fmrml?ref=badge_shield)

[![.github/workflows/main.yml](https://github.com/jdrouet/mrml/actions/workflows/mrml-core-main.yml/badge.svg)](https://github.com/jdrouet/mrml/actions/workflows/mrml-core-main.yml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/main/graph/badge.svg?token=SIOPR0YWZA)](https://codecov.io/gh/jdrouet/mrml)

[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/jdrouet/mrml.svg)](http://isitmaintained.com/project/jdrouet/mrml "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/jdrouet/mrml.svg)](http://isitmaintained.com/project/jdrouet/mrml "Percentage of issues still open")
[![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jdrouet/mrml/maintainability)

## Introduction

This project is a reimplementation of the nice [MJML markup language](https://documentation.mjml.io/) in Rust.

## How to use it in my code

Update your `cargo.toml`:

```toml
[dependencies]
mrml = "3"
serde = { version = "1.0", features = ["derive"] }
```

Create your `main.rs`:

```rust
use mrml::prelude::parser::http_loader::{HttpIncludeLoader, BlockingReqwestFetcher};
use mrml::prelude::parser::ParserOptions;
use mrml::prelude::render::RenderOptions;
use std::collections::HashSet;


fn main() {
  let resolver = HttpIncludeLoader::<BlockingReqwestFetcher>::new_allow(HashSet::from(["http://localhost".to_string()]));
  let parser_options = ParserOptions {
      include_loader: Box::new(resolver),
  };
  let render_options = RenderOptions::default();
  let template = r#"<mjml>
  <mj-body>
    <mj-include path="http://localhost/partials/mj-body.mjml" />
  </mj-body>
</mjml>"#;
  match mrml::parse_with_options(template, &parser_options) {
      Ok(mjml) => match mjml.render(&render_options) {
        Ok(html) => println!("{html}"),
        Err(err) => eprintln!("Couldn't render template: {err:?}"),
      },
      Err(err) => eprintln!("Couldn't parse template: {err:?}"),
  }
}
```

It's also possible to use an async include loader

```rust
use mrml::mj_include::body::MjIncludeBodyKind;
use mrml::prelude::parser::http_loader::{AsyncReqwestFetcher, HttpIncludeLoader};
use mrml::prelude::parser::local_loader::LocalIncludeLoader;
use mrml::prelude::parser::memory_loader::MemoryIncludeLoader;
use mrml::prelude::parser::multi_loader::{MultiIncludeLoader, MultiIncludeLoaderItem, MultiIncludeLoaderFilter};
use mrml::prelude::parser::noop_loader::NoopIncludeLoader;
use mrml::prelude::parser::loader::AsyncIncludeLoader;
use mrml::prelude::parser::AsyncParserOptions;
use mrml::prelude::render::RenderOptions;

#[tokio::main]
async fn main() {
  let resolver = MultiIncludeLoader::<Box<dyn AsyncIncludeLoader + Send + Sync + 'static>>::new()
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
  </mj-body>
</mjml>"#;
  match mrml::async_parse_with_options(json, std::sync::Arc::new(parser_options)).await {
      Ok(mjml) => match mjml.render(&render_options) {
        Ok(html) => println!("{html}"),
        Err(err) => eprintln!("Couldn't render template: {err:?}"),
      },
      Err(err) => eprintln!("Couldn't parse template: {err:?}"),
  }
}
```

## Why?

- A Node.js server rendering an MJML template takes around 20 MB of RAM at startup and 130 MB under stress test. In Rust, less than 1.7 MB at startup and a bit less that 3 MB under stress test.
- The JavaScript implementation cannot be run in the browser; the Rust one (and WebAssembly one) can be.

## You want to contribute?

Feel free to read our [contributing](./contributing.md) section and the [code of conduct](./code-of-conduct.md).

## Performance

With the same Linux amd64 machine, to render the amario template using [hyperfine](https://github.com/sharkdp/hyperfine) (see the script in the `benchmarks` folder).

```
Benchmark 1: mjml /amario.mjml
  Time (mean ± σ):     634.1 ms ±   5.2 ms    [User: 669.3 ms, System: 168.2 ms]
  Range (min … max):   625.8 ms … 642.3 ms    10 runs

Benchmark 2: /usr/bin/mrml /amario.mjml render
  Time (mean ± σ):       5.6 ms ±   0.1 ms    [User: 2.8 ms, System: 2.9 ms]
  Range (min … max):     5.5 ms …   7.1 ms    494 runs

Summary
  /usr/bin/mrml /amario.mjml render ran
  112.83 ± 2.12 times faster than mjml /amario.mjml
```

From this, you can see that `mrml` is **more than 110 faster** than `mjml`.

## Missing implementations

- `mj-style[inline]`: not yet implemented. It requires parsing the generated html to apply the inline styles afterward (that's how it's done in mjml) which would kill the performances. Applying it at render time would improve the performance but it would still require to parse the CSS.

## Who is using MRML?

[<img src="https://www.blizzstatic.com/www/marketing/images/logo.svg" height="22px" />](https://www.blizzfull.com/)

<i>If you are using MRML and want to be added to this list, don't hesitate to create an issue or open a pull request.</i>

## What is using MRML?

[mjml_nif](https://github.com/adoptoposs/mjml_nif) - Elixir library

[mrml-ruby](https://github.com/hardpixel/mrml-ruby) - Ruby library

[mjml-python](https://github.com/mgd020/mjml-python) - Python library

[wagtail-newsletter](https://github.com/wagtail/wagtail-newsletter) - Wagtail extension

<i>If you are using MRML and want to be added to this list, don't hesitate to create an issue or open a pull request.</i>

## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjdrouet%2Fmrml.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjdrouet%2Fmrml?ref=badge_large)
