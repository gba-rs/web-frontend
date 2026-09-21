## About

Web Frontend for the [gba-emu](https://github.com/gba-rs/gba-emu) hosted on github pages [here](https://gba-rs.github.io/web-frontend/)

## Building

### Install `wasm-pack` and `rollup`

```bash
cargo install wasm-pack
npm install --global rollup
```

### Build

Run `make build` to build the wasm package and the bundle. This requires `make`

### Run local

Run `make run` from this repository, then open `http://localhost:8080`.
The development server disables caching so changes to JavaScript, CSS, and WASM
are picked up on reload. For model/CSS edits with an existing build, you can also
run `python dev_server.py 8080` without rebuilding WASM.

The player starts in **2D** mode. Select **3D GBA** or **3D GBA SP** below the screen
to see the console models. If you previously used the old server, restart it and
press **Ctrl+Shift+R** once to clear its cached assets.
