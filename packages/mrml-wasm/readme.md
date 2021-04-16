# MRML Wasm

This project is a reimplementation of the nice `MJML` markup language in Rust, built in webassenbly.

To have more information, take a look at [the repository](https://github.com/jdrouet/mrml).

## Usage on the browser

```js
import { toHtml, toJson, toMjml, validate } from "mrml";
```

## Usage on node

```js
const { toHtml, toJson, toMjml, validate } = require("mrml/node/mrml");
```
