# MRML

[![Crates.io](https://img.shields.io/crates/d/mrml)](https://crates.io/crates/mrml)
![Crates.io](https://img.shields.io/crates/v/mrml)

[![Build Status](https://travis-ci.com/jdrouet/mrml.svg?branch=master)](https://travis-ci.com/jdrouet/mrml)
[![codecov](https://codecov.io/gh/jdrouet/mrml/branch/master/graph/badge.svg?token=L3LKpV3RpR)](https://codecov.io/gh/jdrouet/mrml)

[![Average time to resolve an issue](http://isitmaintained.com/badge/resolution/jdrouet/mrml.svg)](http://isitmaintained.com/project/jdrouet/mrml "Average time to resolve an issue")
[![Percentage of issues still open](http://isitmaintained.com/badge/open/jdrouet/mrml.svg)](http://isitmaintained.com/project/jdrouet/mrml "Percentage of issues still open")
[![Maintainability](https://api.codeclimate.com/v1/badges/7ed23ef670d076ab69a4/maintainability)](https://codeclimate.com/github/jdrouet/mrml/maintainability)

## Introduction

This project is a reimplementation of the nice MJML markup language in Rust.

## How to use it in the cli

```bash
# installing mrml-cli
cargo install --locked mrml-cli
# using it 
mrml-cli path/to/template.mjml validate
mrml-cli path/to/template.mjml render
mrml-cli path/to/template.mjml format-json --pretty
mrml-cli path/to/template.json format-mjml --pretty
# getting some help
mrml-cli --help
```

## Why?

- A Node.js server rendering an MJML template takes around 20 MB of RAM at startup and 130 MB under stress test. In Rust, less than 1.7 MB at startup and a bit less that 3 MB under stress test. The Rust version can also handle twice as many requests per second. You can perform the benchmarks by running `bash script/run-bench.sh`.
- The JavaScript implementation cannot be run in the browser; the Rust one (and WebAssembly one) can be.

## You want to contribute?

Feel free to read our [contributing](./contributing.md) section and the [code of conduct](./code-of-conduct.md).

## Performance

With the same Linux amd64 machine, to render the amario template

- Node: 606.59ms
- Rust: 3.48ms

## You want to sponsor us?

[<img src="https://liberapay.com/assets/liberapay/icon-v2_white-on-yellow.svg?etag=.Z1LYSBJ8Z6GWUeLUUEf2XA~~" height="35px" />](https://liberapay.com/jdrouet/)
[<img src="https://cdn.buymeacoffee.com/buttons/v2/default-yellow.png" alt="Buy Me A Coffee" height="35px" />](https://www.buymeacoffee.com/jdrouet)

<i>Thanks to [zachzurn](https://github.com/zachzurn).</i>
