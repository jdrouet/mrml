# MRML

[![Crates.io](https://img.shields.io/crates/d/mrml)](https://crates.io/crates/mrml)
![Crates.io](https://img.shields.io/crates/v/mrml)

[![Build Status](https://travis-ci.com/jdrouet/mrml.svg?branch=master)](https://travis-ci.com/jdrouet/mrml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/master/graph/badge.svg?token=L3LKpV3RpR)](https://codecov.io/gh/jdrouet/mrml)

[![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jdrouet/mrml/maintainability)

## Introduction

This project is a reimplementation of the nice `MJML` markup language in Rust.

## How to use it in the cli

```bash
# installing mrml-cli
cargo install mrml-cli
# using it 
mrml-cli path/to/template.mjml validate
mrml-cli path/to/template.mjml render
mrml-cli path/to/template.mjml format-json --pretty
mrml-cli path/to/template.json format-mjml --pretty
# getting some help
mrml-cli --help
```

## How to use it in my code

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

## Why?

- A `Node` server rendering a mjml template takes around 20Mo of RAM at startup and 130Mo under stress test. In `Rust`, less than 1.7Mo at startup and a bit less that 3Mo under stress test. The `Rust` version can also handle 2 times more requests per seconds. You can run the bench by doing `bash script/run-bench.sh`.
- The `JS` implementation cannot be run in the browser. In `Rust` (and `Wasm`), you can.

## You want to contribute?

Feel free to read our [contributing](./contributing.md) section and the [code of conduct](./code-of-conduct.md).

## Performance

With the same linux amd64 machine, to render the amario template

- Node: 62.803ms
- Rust: 13.180ms

To reproduce those results:

- Node, in `example/mjml-bench` run `npm start -- ../../resources/template-amario.mjml`
- Rust, run `cargo bench amario`

