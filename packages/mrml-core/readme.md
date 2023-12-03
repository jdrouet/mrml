# MRML

[![Crates.io](https://img.shields.io/crates/d/mrml)](https://crates.io/crates/mrml)
![Crates.io](https://img.shields.io/crates/v/mrml)
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core.svg?type=shield)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core?ref=badge_shield)

[![.github/workflows/main.yml](https://github.com/jolimail/mrml-core/actions/workflows/main.yml/badge.svg)](https://github.com/jolimail/mrml-core/actions/workflows/main.yml)
[![codecov](https://codecov.io/gh/jolimail/mrml-core/branch/main/graph/badge.svg?token=SIOPR0YWZA)](https://codecov.io/gh/jolimail/mrml-core)

[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/jolimail/mrml-core.svg)](http://isitmaintained.com/project/jolimail/mrml-core "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/jolimail/mrml-core.svg)](http://isitmaintained.com/project/jdrouet/mrml "Percentage of issues still open")
[![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jolimail/mrml-core/maintainability)

## Introduction

This project is a reimplementation of the nice [MJML markup language](https://documentation.mjml.io/) in Rust.

## How to use it in my code

Update your `cargo.toml`:

```toml
[dependencies]
mrml = "2"
serde = { version = "1.0", features = ["derive"] }
```

Create your `main.rs`:

```rust
use mrml;

fn main() {
    let root = mrml::parse("<mjml><mj-body></mj-body></mjml>").expect("parse template");
    let opts = mrml::prelude::render::Options::default();
    match root.render(&opts) {
        Ok(content) => println!("{}", content),
        Err(_) => println!("couldn't render mjml template"),
    };
}
```

Available options are:

| Name                 | Comment                                              | Default value                                                                                        |
|----------------------|------------------------------------------------------|------------------------------------------------------------------------------------------------------|
| `disable_comments`   | Strip comments out of rendered HTML                  | `false`                                                                                              |
| `social_icon_origin` | Custom URL for fetching social icons                 | `None`                                                                                               |
| `fonts`              | Default fonts imported in the HTML rendered by MJML  | [See default options](https://github.com/jolimail/mrml-core/blob/main/src/prelude/render.rs#L33-L54) |

## Why?

- A Node.js server rendering an MJML template takes around 20 MB of RAM at startup and 130 MB under stress test. In Rust, less than 1.7 MB at startup and a bit less that 3 MB under stress test. The Rust version can also handle twice as many requests per second. You can perform the benchmarks by running `bash script/run-bench.sh`.
- The JavaScript implementation cannot be run in the browser; the Rust one (and WebAssembly one) can be.

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

[mjml_nif](https://github.com/adoptoposs/mjml_nif) - Elixir library

[mrml-ruby](https://github.com/hardpixel/mrml-ruby) - Ruby library

[mjml-python](https://github.com/mgd020/mjml-python) - Python library

<i>If you are using MRML and want to be added to this list, don't hesitate to create an issue or open a pull request.</i>

## You want to sponsor us?

[<img src="https://liberapay.com/assets/liberapay/icon-v2_white-on-yellow.svg?etag=.Z1LYSBJ8Z6GWUeLUUEf2XA~~" height="35px" />](https://liberapay.com/jdrouet/)
[<img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" height="35px" />](https://www.buymeacoffee.com/jdrouet)

<i>Thanks to [zachzurn](https://github.com/zachzurn).</i>


## License
[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fjolimail%2Fmrml-core?ref=badge_large)
