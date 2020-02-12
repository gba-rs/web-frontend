build:
	wasm-pack build --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

run: build
	python -m http.server 8080