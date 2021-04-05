${HOME}/.cargo/bin/wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

requirements: ${HOME}/.cargo/bin/wasm-pack

format:
	rustfmt --check src/*.rs

clean:
	rm -rf pkg

build-bundler:
	wasm-pack build --out-name mrml --out-dir pkg/bundler --target bundler

build-node:
	wasm-pack build --out-name mrml --out-dir pkg/node --target nodejs

build: clean build-bundler build-node
	mv pkg/node/mrml.d.ts pkg/
	cp license.md pkg/
	cp package.json pkg/
	cp readme.md pkg/
	rm \
		pkg/bundler/license.md pkg/node/license.md \
		pkg/bundler/package.json pkg/node/package.json \
		pkg/bundler/.gitignore pkg/node/.gitignore

publish: build
	wasm-pack publish

test:
	wasm-pack test --node
