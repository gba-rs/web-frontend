build:
	wasm-pack build --release --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

build-dev:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js


run: build
	python -m http.server 8080

run-dev: build-dev
	python -m http.server 8080