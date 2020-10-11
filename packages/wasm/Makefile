clean:
	rm -rf pkg

build: clean
	wasm-pack build --out-name mrml

publish: build
	sed -i pkg/package.json -e 's/mrml-wasm/mrml/g'
	wasm-pack publish
