#!/bin/bash

# cleanup
rm -rf pkg

# build all
for target in web nodejs bundler; do
  wasm-pack build --target $target --out-dir pkg/$target --release
done

# copy resources
cp pkg/nodejs/package.json pkg/
rm pkg/{web,nodejs,bundler}/{.gitignore,LICENSE,package.json,README.md}
cp {LICENSE,README.md} pkg/
node scripts/build-package.js
