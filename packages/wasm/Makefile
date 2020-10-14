NODE_VERSION?=14

${HOME}/.cargo/bin/wasm-pack:
	curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

install-node:
	nvm install ${NODE_VERSION}
	nvm use ${NODE_VERSION}

ci-install-deps: ${HOME}/.cargo/bin/wasm-pack install-node

format:
	rustfmt --check src/*.rs

clean:
	rm -rf pkg

build: clean
	wasm-pack build --out-name mrml

publish: build
	sed -i pkg/package.json -e 's/mrml-wasm/mrml/g'
	wasm-pack publish

test:
	wasm-pack test --node
