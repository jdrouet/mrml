# MRML

[![Build Status](https://travis-ci.com/jdrouet/mrml.svg?branch=master)](https://travis-ci.com/jdrouet/mrml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/master/graph/badge.svg?token=L3LKpV3RpR)](https://codecov.io/gh/jdrouet/mrml)
[![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jdrouet/mrml/maintainability)

## Introduction

This project is a reimplementation of the nice `MJML` markup language in Rust.

## How to use it

```rust
use mrml;

fn main() {
    match mrml::to_html("<mjml><mj-body></mj-body></mjml>", mrml::Options::default()) {
        Ok(content) => println!("{}", content),
        Err(_) => println!("couldn't convert mjml template"),
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

- Node, in `example/mjml-bench` run `npm start -- ../../test/template-amario.mjml`
- Rust, run `cargo bench amario`

> With a web server, as of today, on a mac book pro from 2017. (Old result)
> | | Rust | Node |
> | ---------------------- | ------ | -------- |
> | Requests per seconds | 520.50 | 272.61 |
> | CPU usage at boot time | 0.20% | 1.74% |
> | CPU usage under bench | 49.81% | 136.83% |
> | RAM usage at boot time | 1.12MB | 17.27MB |
> | RAM usage under bench | 2.85MB | 128.32MB |
> | Docker image size | 77.3MB | 178MB |
