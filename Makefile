build:
	rustup run nightly wasm-pack build --release --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

run: build
	python dev_server.py 8080

test: build
	npm test
