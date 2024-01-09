#!/bin/bash

# cleanup
rm -rf pkg

# build all
for target in web nodejs bundler; do
  wasm-pack build --target $target --out-dir pkg/$target --release
done

# copy resources
cp pkg/nodejs/package.json pkg/
rm pkg/{web,nodejs,bundler}/{.gitignore,license.md,package.json,README.md}
cp {license.md,README.md} pkg/
node scripts/build-package.js
