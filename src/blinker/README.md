# Blinker

## Some docs

- https://developer.mozilla.org/ko/docs/WebAssembly/Rust_to_wasm
- https://docs.wasmer.io/integrations/rust
- https://depth-first.com/articles/2020/06/29/compiling-rust-to-webassembly-a-simple-example/

## Prep

```sh
$ curl https://get.wasmer.io -sSfL | sh
$ rustup target add wasm32-unknown-unknown
$ cargo install wasm-pack
$ cargo install wasm-gc # optional
```

## Edit

There are particular things to do make a rust project to be WASM compatible.

## Build

```sh
$ cargo build --target wasm32-unknown-unknown --release

# optional

```

## Run

```sh
$ wasmer run blinker.wasm
```