# MRML Wasm

This project is a reimplementation of the nice `MJML` markup language in Rust, built in webassenbly.

To have more information, take a look at [the repository](https://github.com/jdrouet/mrml).

## Usage on the browser 🌍

```js
import { Engine } from "mrml";

const engine = new Engine();
const result = engine.toHtml("<mjml><mj-body>Hello World</mj-body></mjml>");
```

## Usage on node 💻

```js
const { Engine } = require("mrml/node/mrml");

const engine = new Engine();
const result = engine.toHtml("<mjml><mj-body>Hello World</mj-body></mjml>");
```
