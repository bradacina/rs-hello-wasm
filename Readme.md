# Tetris in WASM using Rust

A Tetris POC in Rust that compiles to WASM (can be played in the browser).

## How to build it

Use `rustup` to install rust (I'm using 1.43 stable).

Add the wasm32 target: `rustup target add wasm32-unknown-unknown`

`./bindgen-cli.sh` (this will install `wasm-bindgen-cli` and `sfz`)

`./build.sh` (this will generate the wasm file and .js bindings inside the `_build` folder)

We use `sfz` to serve files from the `_build` folder. For this just run `./serve.sh` in a separate terminal.

Once `sfz` is running point your browser to `http://localhost:5000/index.html`

After you make some changes to the code, re-run `./build.sh` and refresh the browser page.

## References

MDN docs

web_sys docs

wasm_bindgen docs