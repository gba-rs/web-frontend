build:
	rustup run nightly wasm-pack build --release --target web
	rollup ./main.js --format iife --file ./pkg/bundle.js
	sed -i 's#/pkg/gba_web_frontend_bg.wasm#pkg/gba_web_frontend_bg.wasm#g' pkg/bundle.js

run: build
	python -m http.server 8080