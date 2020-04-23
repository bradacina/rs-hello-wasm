#!/bin/sh
cargo build --target=wasm32-unknown-unknown

wasm-bindgen target/wasm32-unknown-unknown/debug/hello_wasm.wasm --out-dir ./_build --target web

cp -u index.html _build/
cp -u style.css _build/