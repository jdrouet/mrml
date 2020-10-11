# Actix example

This example is a http server rendering templates

## How to run it

```bash
# in root folder
cargo build
# in this folder
cargo run
```

And you will be able to send templates through http.

```bash
curl --data 'template=<mjml><mj-body />' http://localhost:3000/render
```
