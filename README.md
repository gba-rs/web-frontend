## About

Web Frontend for the [gba-emu](https://github.com/gba-rs/gba-emu) hosted on github pages [here](https://gba-rs.github.io/web-frontend/)

## Building

### Install `wasm-pack` and `rollup`

```bash
cargo install wasm-pack
npm install --global rollup
```

### Build

Run `make build` to build the wasm package and the bundle. This requires `make` and `sed`

### Run local

Run `make run`. This will use the python http server to run the website at `http://localhost:8080`