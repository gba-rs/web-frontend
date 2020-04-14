build:
	wasm-pack build --release --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js

run: build
	python -m http.server 8080